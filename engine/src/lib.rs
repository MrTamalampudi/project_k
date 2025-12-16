use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    panic,
    path::PathBuf,
    process::Command,
    time::Duration,
};

use log::{error, info};
use thirtyfour::{DesiredCapabilities, WebDriver, error::WebDriverError};
use webdriver_manager::{WebdriverManager, chrome::ChromeManager};

use ast::{
    testcase::TestCase,
    teststep::{Body, Teststep},
};
use class::{CustomEngine, Method};

mod conditional;
mod custom;
mod element;
mod errors;
mod expression;
mod navigation;
mod shared;
mod timeouts;
mod webdriver;

type Port = u16;
type EngineResult<T> = Result<T, WebDriverError>;

#[macro_export]
macro_rules! e_types {
    () => {
        type Error = WebDriverError;
        type Step = Teststep;
    };
}

#[tokio::main]
pub async fn execute(testcase: &mut TestCase) {
    let mut engine = Engine::new(testcase).await;
    if let Err(error) = engine.start().await {
        error!("{:#?}", error);
    }
    engine.kill();
}

pub struct Engine<'a> {
    testcase: &'a mut TestCase,
    driver: WebDriver,
    port: Port,
}

impl<'a> Engine<'a> {
    async fn new(testcase: &'a mut TestCase) -> Engine<'a> {
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

    async fn start(&mut self) -> EngineResult<()> {
        let body = self.testcase.body.clone();
        self.execute_body(body).await?;
        Ok(())
    }

    #[inline]
    async fn execute_body(&mut self, body: Body) -> EngineResult<()> {
        for teststep in body.teststeps.iter() {
            match teststep {
                Teststep::Action(step) => match step.method {
                    Method::WEB_DRIVER(_) => self.webdriver(&teststep).await?,
                    Method::ELEMENT(_) => self.element(&teststep).await?,
                    Method::NAVIGATION(_) => self.navigation(&teststep).await?,
                    Method::TIMEOUTS(_) => self.timeouts(&teststep).await?,
                    Method::CUSTOM(_) => self.custom(&teststep).await?,
                    _ => {}
                },
                Teststep::VarDecl(_) => self.VAR_DECLARATION(&teststep).await?,
                Teststep::If(_) => self.conditional(&teststep).await?,
                _ => {}
            };
        }
        Ok(())
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
    let manager = ChromeManager::new();
    let mut driver_path: Option<PathBuf> = None;
    if let Ok(mut manager) = manager {
        let driver_path_ = tokio::task::spawn_blocking(move || {
            let browser_version = manager.discover_browser_version().unwrap_or_default();
            if let Some(version) = browser_version {
                info!("Discoverd Chrome {}", &version);
                manager.set_browser_version(version);
            };
            let driver_version = manager.request_driver_version();
            if let Ok(version) = driver_version {
                manager.set_driver_version(version);
            }

            let path = manager.get_driver_path_in_cache().ok().expect("");
            if path.exists() {
                Some(path)
            } else {
                if let Ok(_) = manager.download_driver() {
                    Some(path)
                } else {
                    None
                }
            }
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
    //time may vary from machine to machine & 3sec for my machine
    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(port)
}

fn pick_a_free_port() -> Option<Port> {
    let ip4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    Some(TcpListener::bind(ip4).ok()?.local_addr().ok()?.port())
}
