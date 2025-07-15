use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    ops::Deref,
    panic,
    path::PathBuf,
    process::Command,
    time::Duration,
};

use thirtyfour::{By, DesiredCapabilities, WebDriver};
use webdriver_manager::{chrome::ChromeManager, logger::Logger, WebdriverManager};

use crate::{
    ast::{
        arguments::{Args, URL_ARGKEY},
        testcase::{TestCase, TestcaseBody},
        teststep::TestStep,
    },
    class::{ElementEngine, Method, ELEMENT, NAVIGATION, WEB_DRIVER as Driver},
    engine::element::Element,
    parser::locator::LocatorStrategy,
};

mod element;
mod navigation;

type Port = u16;

#[tokio::main]
pub async fn execute(testcase: TestCase) {
    let mut engine = Engine::new(testcase).await;
    engine.start().await;
    engine.kill();
}

pub struct Engine {
    testcase: TestCase,
    driver: WebDriver,
    port: Port,
}

impl Engine {
    async fn new(testcase: TestCase) -> Engine {
        let (driver, port) = match create_web_driver().await {
            Ok((driver, port)) => (driver, port),
            Err(error) => panic!("{error}"),
        };
        Engine {
            testcase,
            driver,
            port,
        }
    }

    fn kill(&self) {
        let mut process = Command::new("sh");
        process.arg("-c");
        process.arg(format!("kill -9 `lsof -t -i:{}`", self.port));
        match process.spawn() {
            Ok(_) => println!("Successfully killed driver process"),
            Err(_) => println!("Failed to killed driver process"),
        };
    }

    async fn start(&mut self) {
        while let Some(step_ref_cell) = self.testcase.test_step.take() {
            let step = step_ref_cell.borrow();

            match step.deref() {
                TestcaseBody::TESTSTEP(stepo) => {
                    match stepo.method {
                        Method::WEB_DRIVER(Driver::NAVIGATE) => self.navigate(stepo).await,
                        Method::ELEMENT(ELEMENT::CLICK) => {
                            let element = Element {
                                driver: &mut self.driver,
                            };
                            element.CLICK(stepo).await;
                        }
                        Method::NAVIGATION(NAVIGATION::BACK) => self.back(stepo).await,
                        Method::NAVIGATION(NAVIGATION::FORWARD) => self.forward(stepo).await,
                        _ => {}
                    }
                    if let Some(next_step) = &stepo.next {
                        self.testcase.test_step = Some(next_step.clone());
                    } else {
                        self.testcase.test_step = None;
                    }
                }
                _ => {
                    self.testcase.test_step = None;
                }
            };
        }
    }

    async fn navigate(&self, teststep: &TestStep) {
        let url = teststep.arguments.get(URL_ARGKEY).unwrap();
        if let Args::String(url) = url {
            if let Err(_) = self.driver.goto(url).await {
                panic!("there is an error");
            }
        };
    }

    async fn back(&self, teststep: &TestStep) {
        if let Ok(_) = self.driver.back().await {
            println!("back done");
        };
    }

    async fn forward(&self, teststep: &TestStep) {
        self.driver.forward().await;
    }
}

async fn create_web_driver() -> Result<(WebDriver, Port), String> {
    let port = start_driver().await?;
    let caps = DesiredCapabilities::chrome();
    let server = format!("http://localhost:{port}");
    let driver = WebDriver::new(server, caps).await;

    match driver {
        Ok(driver) => Ok((driver, port)),
        Err(error) => Err(error.to_string()),
    }
}

async fn manage_browser_driver() -> Result<Option<PathBuf>, String> {
    let log = Logger::create("/home/manikanta-reddy/new.json", true, true, "");
    let manager = ChromeManager::new();
    let mut driver_path: Option<PathBuf> = None;
    if let Ok(mut manager) = manager {
        let driver_path_ = tokio::task::spawn_blocking(move || {
            manager.set_logger(log);
            let browser_version = manager.discover_browser_version().unwrap_or_default();
            if let Some(version) = browser_version {
                manager.set_browser_version(version);
            };
            let driver_version = manager.request_driver_version();
            if let Ok(version) = driver_version {
                manager.set_driver_version(version);
            }
            if let Err(error) = manager.download_driver() {
                println!("Error {error}");
            }

            let path = if let Ok(path) = manager.get_driver_path_in_cache() {
                Some(path)
            } else {
                None
            };
            path
        })
        .await;

        if let Ok(path) = driver_path_ {
            driver_path = path;
        }
    } else {
        return Err(format!("Error while setuping driver"));
    }

    Ok(driver_path)
}

async fn start_driver() -> Result<Port, String> {
    let mut command = Command::new("sh");
    command.arg("-c");
    let port = pick_a_free_port().unwrap();
    if let Ok(driver_path) = manage_browser_driver().await {
        command.arg(format!(".{:?} --port={port}", driver_path.unwrap()));
        command.current_dir("/");
        command.spawn().expect("msg");
    } else {
        return Err(format!("Error while setuping driver"));
    }

    //to start chromedriver it will take a sec or two sec time till then sleep
    // time may vary from machine to machine & 3sec for my machine
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(port)
}

fn pick_a_free_port() -> Option<Port> {
    let ip4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    Some(TcpListener::bind(ip4).ok()?.local_addr().ok()?.port())
}
