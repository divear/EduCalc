use dotenvy::dotenv;
use std::error::Error;
// mod prev;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::env;
use std::time::Duration;

pub fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    dotenv().expect(".env file not found");
    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");

    println!("signing in");
    tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
    tab.wait_for_element("input#home_Login_2e1")?.click()?;
    tab.type_str(&username)?;
    tab.wait_for_element("input#home_Login_2e2")?.click()?;
    tab.type_str(&password)?.press_key("Enter")?;

    println!("znamky page");
    tab.navigate_to("https://sspbrno.edupage.org/znamky")?
        .wait_for_element("#edubarStartButton")?;
    // tab.wait_for_element(".fixedCell")?; //delete if this works
    println!("start");
    let inner_text_content = tab
        .wait_for_element_with_custom_timeout(".znZnamka", Duration::from_secs(15))?
        .get_inner_text()?;
    println!("{}", inner_text_content);
    println!("end");
    // let test_button = tab
    //     .find_elements("znznamka")?
    //     .capture_screenshot(page::capturescreenshotformatoption::png)?;
    // std::fs::write("button.jpeg", test_button)?;
    let mut znamky_all: Vec<usize> = Vec::new();
    let containing_element = tab.find_elements(".znZnamka")?;
    for i in &containing_element {
        println!("{:?}", i);
        let new_znamka = i.get_inner_text();
        // println!("new znamka: {:?}", new_znamka);
        // let znamka_int: usize = new_znamka?.parse()?;

        match new_znamka {
            Ok(new_znamka) => {
                println!("new znamka: {:?}", new_znamka);
                match new_znamka.parse::<usize>() {
                    Ok(znamka_int) => {
                        println!("Parsed number: {:?}", znamka_int);
                        znamky_all.push(znamka_int);
                        // You can use `znamka_int` here.
                    }
                    Err(_) => {
                        println!("Failed to parse '{}' as a number", new_znamka);
                        // Handle the case where the string is not a valid number.
                    }
                }
            }
            Err(e) => {
                println!("Failed to get inner text: {:?}", e);
                // Handle the error case where `get_inner_text` failed.
            }
        }
        // znamky_all.push(znamka_int);
    }
    // let inner_divs = containing_element.find_elements(".znznamka")?;
    println!("{:?}", znamky_all);

    // let curr_prumer = tab.wait_for_element(".expandImg")?.get_inner_text()?;
    // println!("{}", curr_prumer);

    Ok(())
}
