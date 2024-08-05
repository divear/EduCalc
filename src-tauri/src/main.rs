// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use colored::Colorize;
use headless_chrome::Browser;
use std::{collections::HashSet, env, error::Error, fmt, time::Duration};
mod term;
const WAIT_LIMIT: u64 = 15;
struct ZnamkaStruct {
    predmet: String,
    znamka: f32,
    vaha: f32,
}
impl fmt::Display for ZnamkaStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        println!(
            "[\"{}\",\"{}\",\"{}\"]",
            self.predmet, self.znamka, self.vaha
        );
        write!(
            f,
            "[\"{}\",\"{}\",\"{}\"]",
            self.predmet, self.znamka, self.vaha
        )
    }
}

// the individual processing functions
fn process_two(znamka: &str) -> Option<f32> {
    // 2- etc.
    let returned = znamka.chars().nth(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    // println!("{:?}", returned_num);
    Some(returned_num + 0.5)
}
fn process_longer(znamka: &str) -> Option<f32> {
    // 9 / 15 = 60% → 3 etc.
    let returned = znamka.chars().nth_back(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    Some(returned_num)
}
fn process_percent(znamka: &str) -> Option<f32> {
    // 100% etc.
    let znamka: f32 = znamka.replace("%", "").parse().expect("a proper float");
    Some(znamka)
}

#[tauri::command]
fn subjects(username: &str, password: &str) -> (String, String) {
    match get_subjects(username, password) {
        Ok((subjects, grades)) => {
            println!("success");
            (subjects, grades)
        }
        Err(err) => {
            println!("bad data {}", err);
            (
                String::from("could not get subjects"),
                String::from("could not get grades"),
            )
        }
    }
}
fn get_subjects(username: &str, password: &str) -> Result<(String, String), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    loop {
        tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
        tab.wait_for_element("input#home_Login_2e1")?.click()?;
        tab.type_str(&username)?;
        tab.wait_for_element("input#home_Login_2e2")?.click()?;
        tab.type_str(&password)?.press_key("Enter")?;

        match tab.wait_for_element(".user-button-icon-outer") {
            Ok(_) => {
                println!("Signed in successfully!")
            }
            Err(_) => {
                println!("Wrong credentials");
                let _ = main();
            }
        };
        // println!("finished signin");

        tab.navigate_to("https://sspbrno.edupage.org/znamky/?eqa=d2hhdD1zdHVkZW50dmlld2VyJnBvaGxhZD1wb2RsYURhdHVtdSZ6bmFta3lfeWVhcmlkPTIwMjMmem5hbWt5X3llYXJpZF9ucz0xJm5hZG9iZG9iaWU9UDImcm9rb2Jkb2JpZT0yMDIzJTNBJTNBUDImZG9ScT0xJndoYXQ9c3R1ZGVudHZpZXdlciZ1cGRhdGVMYXN0Vmlldz0w")?;
        println!("Getting the grades...");
        match tab.wait_for_element_with_custom_timeout(
            "#edubarStartButton",
            Duration::from_secs(WAIT_LIMIT),
        ) {
            Ok(_) => {
                break;
            }
            Err(_) => {
                println!("Grades not found; trying again");
                let _ = main();
            }
        };
    }
    let mut everything_vec: Vec<ZnamkaStruct> = vec![];
    let everything = tab.find_elements(".app-list-item-main")?;

    for i in everything {
        let inner_text = i.get_inner_text()?;
        let all: Vec<&str> = inner_text.lines().collect();

        // println!("{:?}", all);
        let new_znamka = all[2];
        if all[0] == "Chování" || all[1] == "Vysvědčení" {
            continue;
        }

        let created_znamka = match new_znamka.parse::<f32>() {
            Ok(znamka_int) => {
                println!("Parsed grade: {}", znamka_int.to_string().green());
                Ok(znamka_int)
            }
            Err(_) => {
                // println!("'{}' není v normálním formátu", new_znamka.yellow());
                if new_znamka.chars().nth_back(0) == Some('%') {
                    let extracted_znamka = process_percent(&new_znamka);
                    if extracted_znamka.is_some() {
                        Ok(extracted_znamka.expect("adding a working grade failed"))
                    } else {
                        Err("extracted_znamka doesnt exist")
                    }
                } else {
                    // println!("new_znamka: {:?}", &new_znamka);
                    let extracted_znamka = match new_znamka.len() {
                        1 => {
                            // println!("+/-/o/S se nevztahuje na prumer");
                            None
                        }
                        2 => process_two(&new_znamka),
                        3.. => process_longer(&new_znamka),
                        _ => {
                            println!("the length of grade is 0 or negative!");
                            None
                        }
                    };
                    if extracted_znamka.is_none() {
                        Err("this grade was not counted into the average")
                    } else {
                        // println!("{:?}", extracted_znamka);
                        Ok(extracted_znamka.expect("adding a working grade failed"))
                    }
                }
            }
        };
        match created_znamka {
            Ok(created_znamka) => {
                if let Some(start) = all[1].find('∙') {
                    if let Some(end) = all[1].find('×') {
                        let start_pos = start + '∙'.len_utf8();
                        let result = &all[1][start_pos..end].trim();
                        let vaha = result.parse().unwrap();
                        // println!("vaha: {}", vaha);
                        let new_znamka_instance = ZnamkaStruct {
                            predmet: all[0].to_string(),
                            znamka: created_znamka,
                            vaha,
                        };
                        // println!("vaha saved: {}", new_znamka_instance.vaha);
                        // println!("znamka saved: {}", new_znamka_instance.znamka);
                        everything_vec.push(new_znamka_instance);
                    } else {
                        println!("The '×' character was not found.");
                    }
                } else {
                    println!("The '∙' character was not found.");
                }
            }
            Err(_) => {
                // println!("error: the created_znamka {:?} isn't valid", created_znamka);
            }
        };
        // println!("{:?}", created_znamka);
    }
    print!("{}[2J", 27 as char); //clear the screen
    let mut set_existujicich_predmetu = HashSet::new();
    for i in &everything_vec {
        // println!("{:?}", &i);
        set_existujicich_predmetu.insert(&i.predmet);
    }
    let mut vec_predmetu = Vec::from_iter(set_existujicich_predmetu);
    vec_predmetu.sort();
    Ok((
        vec_predmetu
            .iter()
            .map(|x| x.to_string() + ",")
            .collect::<String>(),
        everything_vec
            .iter()
            .map(|x| x.to_string() + ",")
            .collect::<String>(),
    ))
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"--term".to_string()) {
        let _ = term::term();
    } else {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![subjects])
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
