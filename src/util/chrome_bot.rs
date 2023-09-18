use crate::config::Config;
use base64;
use chrono::prelude::{DateTime, Local};
use headless_chrome::Tab;
use headless_chrome::{types::PrintToPdfOptions, Browser, LaunchOptions};
use std::error::Error;
use std::fs::File;
use std::io::{Write, self};
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tesseract::Tesseract;
lazy_static! {
    static ref CONFIG: Config<'static> = Config::load();
}

pub struct ChromeBot {
    browser: Browser,
    tab: Arc<Tab>,
}

impl ChromeBot {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let browser = Browser::new(LaunchOptions {
            headless: true,
            sandbox: false,
            idle_browser_timeout: Duration::from_secs(30),
            window_size: None,
            path: None,
            user_data_dir: None,
            port: None,
            ignore_certificate_errors: true,
            extensions: Vec::new(),
            process_envs: None,
            #[cfg(feature = "fetch")]
            fetcher_options: Default::default(),
            args: Vec::new(),
            disable_default_args: false,
            proxy_server: None,
        })?;
        let tab = browser.new_tab()?;
        Ok(Self { browser, tab })
    }
    pub fn download_report(&self) -> Result<(), Box<dyn Error>> {
        self.check_auth_code()?;
        self.login()?;
        self.download_daily_report()?;
        self.download_monthly_report()?;
        self.download_weekly_report()?;
        Ok(())
    }
    fn check_auth_code(&self) -> Result<(), Box<dyn Error>> {
        let mut correct_code = "".to_string();
        while correct_code == "".to_string() {
            self.tab.navigate_to("https://www.pmotcloud.org.tw/")?;
            self.tab.wait_until_navigated()?;
            let elem = self.tab.wait_for_element("canvas#auth-code")?;
            let auth_image = elem
                .call_js_fn("function(){ return this.toDataURL() }", vec![], false)?
                .value;
            let code = match auth_image {
                Some(data) => {
                    let code = Self::recognize_auth_code(data.to_string());
                    code
                }
                None => return Err("Auth code image not found".into()),
            };

            self.tab.wait_for_element("input#validText")?.click()?;
            self.tab.type_str(&code)?.press_key("Enter")?;
            let v = self.tab.find_element("#verify_code > input")?.call_js_fn(
                "function(){ return this.value }",
                vec![],
                false,
            )?;
            match v.value {
                Some(data) => {
                    println!("auth code {}", data.to_string());
                    if data.to_string().replace("\\", "").replace("\"", "") == "checked".to_string()
                    {
                        correct_code = code.clone();
                    }
                }
                None => {}
            }
        }
        Ok(())
    }
    fn login(&self) -> Result<(), Box<dyn Error>> {
        self.tab.wait_for_element("input#account")?.click()?;
        self.tab.type_str("iisi_b")?;
        self.tab.wait_for_element("input#password")?.click()?;
        self.tab.type_str("1qaz2wsx#EDC")?;
        self.tab.wait_for_element("button#login")?.click()?;
        thread::sleep(Duration::from_millis(5000));
        Ok(())
    }
    fn download_daily_report(&self) -> Result<(), Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        self.tab
            .navigate_to("https://www.pmotcloud.org.tw/backstage/moea_E3.html")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        self.tab
            .navigate_to("https://www.pmotcloud.org.tw/backstage/moea_E3.html")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        self.tab
            .navigate_to("https://dashboard.pmotcloud.org.tw/cloud/e1/print?report=day")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        let pdf = self.tab.print_to_pdf(Some(PrintToPdfOptions::default()))?;
        let mut file = File::create(format!(
            "report/daily/daily-{}.pdf",
            now.format("%Y%m%d%H%M")
        ))?;
        file.write_all(&pdf)?;
        return Ok(());
    }

    fn download_weekly_report(&self) -> Result<(), Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        self.tab
            .navigate_to("https://www.pmotcloud.org.tw/backstage/moea_E3.html")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        self.tab
            .navigate_to("https://www.pmotcloud.org.tw/backstage/moea_E3.html")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        self.tab
            .navigate_to("https://dashboard.pmotcloud.org.tw/cloud/e1/print?report=week")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        let pdf = self.tab.print_to_pdf(Some(PrintToPdfOptions::default()))?;
        let mut file = File::create(format!(
            "report/weekly/weekly-{}.pdf",
            now.format("%Y%m%d%H%M")
        ))?;
        file.write_all(&pdf)?;
        return Ok(());
    }

    fn download_monthly_report(&self) -> Result<(), Box<dyn Error>> {
        let now: DateTime<Local> = Local::now();
        self.tab
            .navigate_to("https://www.pmotcloud.org.tw/backstage/moea_E3.html")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        self.tab
            .navigate_to("https://www.pmotcloud.org.tw/backstage/moea_E3.html")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        self.tab
            .navigate_to("https://dashboard.pmotcloud.org.tw/cloud/e1/print?report=month")?;
        self.tab.wait_until_navigated()?;
        thread::sleep(Duration::from_millis(5000));
        let pdf = self.tab.print_to_pdf(Some(PrintToPdfOptions::default()))?;
        let mut file = File::create(format!(
            "report/monthly/monthly-{}.pdf",
            now.format("%Y%m%d%H%M")
        ))?;
        file.write_all(&pdf)?;
        return Ok(());
    }
    fn recognize_auth_code(image_base64: String) -> String {
        let temp = image_base64
            .replace("\\", "")
            .replace("\"", "")
            .replace("data:image/png;base64,", "");
        let image = base64::decode(temp).unwrap();
        let result = Tesseract::new(None, Some("eng"))
            .unwrap()
            .set_image_from_mem(&image)
            .unwrap()
            .get_text()
            .unwrap();
        result.replace(" ", "")
    }
    fn kill_process(&self) -> Result<(), Box<dyn Error>> {
        let process_id = self.browser.get_process_id();
        if let Some(id) = process_id {
            let result = Command::new("kill")
                .arg("-9")
                .arg(id.to_string())
                .output()?;
            io::stderr().write_all(&result.stderr)?;
            io::stderr().write_all(b"\n")?;
        }
        Ok(())
    }
}
