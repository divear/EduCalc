use colored::Colorize;
use dotenvy::dotenv;
// mod prev;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::{env, error::Error, fs, /* thread,*/ time::Duration};

const WAIT_LIMIT: u64 = 15;

fn process_two(znamka: &str) -> Option<f32> {
    // 2- etc.
    println!("{} passed into process_two", znamka);
    let returned = znamka.chars().nth(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    println!("{:?}", returned_num);
    Some(returned_num + 0.25)
}
fn process_longer(znamka: &str) -> Option<f32> {
    // 9 / 15 = 60% → 3 etc.
    println!("{} passed into process_longer", znamka);

    let returned = znamka.chars().nth_back(0).unwrap().to_string();
    let returned_num: f32 = returned.parse().expect("a proper float");
    println!("{:?}", returned_num);
    Some(returned_num)
}
fn process_percent(znamka: &str) -> Option<f32> {
    // 100% etc.
    println!("{} passed into process_percent", znamka);
    let znamka: f32 = znamka.replace("%", "").parse().expect("a proper float");
    println!("{}", znamka);
    Some(znamka)
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    dotenv().expect(".env file not found - follow instructions in the README");
    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");

    println!("signing in");
    tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
    tab.wait_for_element("input#home_Login_2e1")?.click()?;
    tab.type_str(&username)?;
    tab.wait_for_element("input#home_Login_2e2")?.click()?;
    tab.type_str(&password)?.press_key("Enter")?;

    println!("znamky page");
    tab.navigate_to("https://sspbrno.edupage.org/znamky/?eqa=d2hhdD1zdHVkZW50dmlld2VyJnBvaGxhZD1wb2RsYURhdHVtdSZ6bmFta3lfeWVhcmlkPTIwMjMmem5hbWt5X3llYXJpZF9ucz0xJm5hZG9iZG9iaWU9UDImcm9rb2Jkb2JpZT0yMDIzJTNBJTNBUDImZG9ScT0xJndoYXQ9c3R1ZGVudHZpZXdlciZ1cGRhdGVMYXN0Vmlldz0w")?;

    loop {
        match tab.wait_for_element_with_custom_timeout(
            "#edubarStartButton",
            Duration::from_secs(WAIT_LIMIT),
        ) {
            Ok(d) => {
                Some(d);
                break;
            }
            Err(_) => {
                println!("could not find, trying again");
            }
        };
    }

    // }
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Png, None, None, true)?;
    // Save the screenshot to disc
    fs::write("./assets/screenshot.png", jpeg_data)?;

    println!("start");
    let inner_text_content = tab
        .wait_for_element_with_custom_timeout(".znZnamka", Duration::from_secs(WAIT_LIMIT))?
        .get_inner_text()?;
    println!("{}", inner_text_content);
    println!("end");
    let mut znamky_all: Vec<f32> = Vec::new();
    let containing_element = tab.find_elements(".znZnamka")?;
    for i in &containing_element {
        println!("{:?}", i);
        let new_znamka = i.get_inner_text();

        match new_znamka {
            Ok(new_znamka) => {
                println!("new znamka: {:?}", new_znamka);
                match new_znamka.parse::<f32>() {
                    Ok(znamka_int) => {
                        println!("Parsed number: {}", znamka_int.to_string().green());
                        znamky_all.push(znamka_int);
                    }
                    Err(_) => {
                        println!("'{}' není v normálním formátu", new_znamka.yellow());
                        if new_znamka.chars().nth_back(0) == Some('%') {
                            let extracted_znamka = process_percent(&new_znamka);
                            if extracted_znamka.is_some() {
                                znamky_all
                                    .push(extracted_znamka.expect("adding a working grade failed"))
                            }
                        } else {
                            let extracted_znamka = match new_znamka.len() {
                                1 => {
                                    println!("+/-/o/S se nevztahuje na prumer");
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
                                println!("this grade was not counted into the average")
                            } else {
                                znamky_all
                                    .push(extracted_znamka.expect("adding a working grade failed"));
                                println!("{:?}", extracted_znamka);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Failed to get inner text: {:?}", e);
                // Handle the error case where `get_inner_text` failed.
            }
        }
    }
    println!("{:?}", znamky_all);
    let grades_as_string = znamky_all
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    fs::write("./assets/last_grades.txt", grades_as_string).expect("unable to write in file"); // in case it fails next time
    let global_average: f32 =
        &znamky_all.clone().into_iter().sum::<f32>() / znamky_all.len() as f32;
    println!("global average grade: {:?}", global_average);

    Ok(())
}
