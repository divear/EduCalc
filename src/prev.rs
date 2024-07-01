use dotenvy::dotenv;
use rand::Rng;
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env file not found");

    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");

    // Initialize HTTP client with cookie store
    let client = Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::limited(10)) // Follow up to 10 redirects
        .build()?;

    // Open the login page
    let login_url = "ttps://sspbrno.edupage.org/login/edubarLogin.php";
    let res = client.get(login_url).send()?;
    let body = res.text()?;
    // println!("Initial page body:\n{}", body); // Debugging line to check the content of the initial page

    // Parse the login page to extract the CSRF token
    let document = Html::parse_document(&body);
    let selector = Selector::parse("input[name=csrfauth]").unwrap();
    let csrf_token = document
        .select(&selector)
        .next()
        .and_then(|n| n.value().attr("value"))
        .ok_or("CSRF token not found")?;

    // Prepare login credentials
    let mut form_data = HashMap::new();
    form_data.insert("csrfauth", csrf_token.to_string());
    form_data.insert("username", username);
    form_data.insert("password", password);

    // Submit the form
    let login_res = client.post(login_url).form(&form_data).send()?;
    if !login_res.status().is_success() {
        return Err("Login failed".into());
    }
    let login_body = login_res.text()?;
    println!("Login response body:\n{}", login_body); // Debugging line to check the response after login
    sleep(Duration::from_secs(5));
    // Access the grades page
    let znamky_url = "https://sspbrno.edupage.org/znamky/";
    let znamky_res = client.get(znamky_url).send()?;
    if !znamky_res.status().is_success() {
        return Err("Failed to access the grades page".into());
    }

    let znamky_body = znamky_res.text()?;
    let html_doc = Html::parse_document(&znamky_body);
    let class_selector = Selector::parse(".edubarSidemenu").unwrap();
    println!("Grades page body:\n{}", znamky_body); // Debugging line to check the content of the grades page
    for element in html_doc.select(&class_selector) {
        let link_selector = Selector::parse("a").unwrap();

        for link in element.select(&link_selector) {
            if let Some(href) = link.value().attr("href") {
                println!("{}", href);
            }
        }
    }
    // Calculate the average of random numbers
    let prumer = calc_prumer();
    println!("Calculated average: {}", prumer);

    Ok(())
}

fn calc_prumer() -> usize {
    let mut znamky: Vec<usize> = Vec::new();
    for _i in 0..10 {
        let randint = rand::thread_rng().gen_range(0..100);
        znamky.push(randint);
    }
    let prumer = znamky.iter().sum::<usize>() / znamky.len();
    prumer
}
