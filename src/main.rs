use dotenvy::dotenv;
use std::error::Error;
mod prev;
use headless_chrome::protocol::cdp::Page;
use headless_chrome::Browser;
use std::env;

pub fn main() -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    dotenv().expect(".env file not found");
    let username = env::var("USERNAME").expect("USERNAME environment variable not set");
    let password = env::var("PASSWORD").expect("PASSWORD environment variable not set");

    tab.navigate_to("https://sspbrno.edupage.org/login/edubarLogin.php")?;
    tab.wait_for_element("input#home_Login_2e1")?.click()?;
    tab.type_str(&username)?;
    tab.wait_for_element("input#home_Login_2e2")?.click()?;
    tab.type_str(&password)?.press_key("Enter")?;
    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    std::fs::write("screenshot2.jpeg", jpeg_data)?;
    tab.navigate_to("https://sspbrno.edupage.org/znamky")?;
    let elem = tab
        .wait_for_element("span.tips")?
        .capture_screenshot(Page::CaptureScreenshotFormatOption::Png)?;
    // std::fs::write("elem.jpeg", elem)?;

    let jpeg_data =
        tab.capture_screenshot(Page::CaptureScreenshotFormatOption::Jpeg, None, None, true)?;
    std::fs::write("screenshot3.jpeg", jpeg_data)?;

    // let remote_object = elem.call_js_fn(
    //     r#"
    //     function getIdTwice () {
    //         // `this` is always the element that you called `call_js_fn` on
    //         const id = this.id;
    //         return id + id;
    //     }
    // "#,
    //     vec![],
    //     false,
    // )?;
    // match remote_object.value {
    //     Some(returned_string) => {
    //         dbg!(&returned_string);
    //         assert_eq!(returned_string, "firstHeadingfirstHeading".to_string());
    //     }
    //     _ => unreachable!(),
    // };

    Ok(())
}
