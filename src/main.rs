use dotenvy::dotenv;
use headless_chrome::Browser;
use std::{collections::HashSet, env, error::Error, io, io::prelude::*, time::Duration};

const WAIT_LIMIT: u64 = 15;
struct ZnamkaStruct {
    predmet: String,
    znamka: f32,
    vaha: f32,
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
fn prompt_and_read(prompt: &str) -> Result<usize, Box<dyn std::error::Error>> {
    print!("{}", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input)?;
    Ok(input.trim().parse()?)
}
fn prompt_and_read_creds(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .lock()
        .read_line(&mut input)
        .expect("failed to read line from user");
    input.trim().to_string()
}

pub fn main() -> Result<(), Box<dyn Error>> {
    println!("starting script");
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    let username: String;
    let password: String;
    match dotenv() {
        Ok(..) => {
            println!("loaded .env file");
            username = env::var("USERNAME").expect("USERNAME environment variable not set");
            password = env::var("PASSWORD").expect("PASSWORD environment variable not set");
        }
        Err(..) => {
            println!(".env doesn't exist");
            username = prompt_and_read_creds("Your EduPage username: ").to_string();
            password = prompt_and_read_creds("Your EduPage password: ").to_string();
        }
    }

    loop {
        tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
        tab.wait_for_element("input#home_Login_2e1")?.click()?;
        tab.type_str(&username)?;
        tab.wait_for_element("input#home_Login_2e2")?.click()?;
        tab.type_str(&password)?.press_key("Enter")?;

        println!("finished signin");

        tab.navigate_to("https://sspbrno.edupage.org/znamky/?eqa=d2hhdD1zdHVkZW50dmlld2VyJnBvaGxhZD1wb2RsYURhdHVtdSZ6bmFta3lfeWVhcmlkPTIwMjMmem5hbWt5X3llYXJpZF9ucz0xJm5hZG9iZG9iaWU9UDImcm9rb2Jkb2JpZT0yMDIzJTNBJTNBUDImZG9ScT0xJndoYXQ9c3R1ZGVudHZpZXdlciZ1cGRhdGVMYXN0Vmlldz0w")?;
        println!("navigated to znamky page");
        match tab.wait_for_element_with_custom_timeout(
            "#edubarStartButton",
            Duration::from_secs(WAIT_LIMIT),
        ) {
            Ok(d) => {
                Some(d);
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
                // println!("Parsed number: {}", znamka_int.to_string().green());
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
                println!("error: the created_znamka isn't valid");
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
    for (i, p) in vec_predmetu.clone().into_iter().enumerate() {
        println!("{}) - {}", i, p);
    }
    for _ in 1.. {
        let predmet_pick_index = prompt_and_read("Choose subject: ")?;
        // random order, because HashSet
        let predmet_pick = &vec_predmetu[predmet_pick_index];
        println!("\nYou chose: {}", predmet_pick);
        let mut picked_predmet_znamky: Vec<f32> = vec![];
        let mut picked_predmet_vahy: Vec<f32> = vec![];
        for i in &everything_vec {
            if i.predmet == **predmet_pick {
                picked_predmet_znamky.push(i.znamka * i.vaha);
                picked_predmet_vahy.push(i.vaha);
            }
        }
        // println!("{:?}", x);
        // println!("{:?}", picked_predmet_vahy);
        // println!("{:?}", picked_predmet_znamky);
        println!(
            "Your current average: {}",
            picked_predmet_znamky.clone().into_iter().sum::<f32>()
                / picked_predmet_vahy.clone().into_iter().sum::<f32>()
        );
        let nova_znamka: f32 = prompt_and_read("Add new grade: ")? as f32;
        let nova_vaha: f32 = prompt_and_read("The grade's weight: ")? as f32;
        println!("New grade: {}, Weight: {}", nova_znamka, nova_vaha);
        picked_predmet_znamky.push(nova_znamka * nova_vaha);
        picked_predmet_vahy.push(nova_vaha);
        println!(
            "Your new calculated average: {}\n",
            picked_predmet_znamky.clone().into_iter().sum::<f32>()
                / picked_predmet_vahy.clone().into_iter().sum::<f32>()
        );
    }
    Ok(())
}
