use std::{
    net::{Ipv4Addr, SocketAddrV4, TcpListener},
    path::{Path, PathBuf},
    process::Command,
    time::Duration,
};

use thirtyfour::{DesiredCapabilities, WebDriver};
use webdriver_manager::{chrome::ChromeManager, logger::Logger, WebdriverManager};

type Port = u16;

#[tokio::main]
pub async fn engine() {
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
    };
    let mut command = Command::new("sh");
    command.arg("-c");
    let port = pick_a_free_port().unwrap();
    println!("porttttttttt={port}");
    command.arg(format!(".{:?} --port={port}", driver_path.unwrap()));
    command.current_dir("/");
    command.spawn().expect("msg");

    //to start chromedriver it will take a sec or two sec time till then sleep
    tokio::time::sleep(Duration::from_secs(3)).await;

    let caps = DesiredCapabilities::chrome();
    let server = format!("http://localhost:{port}");
    println!("serverrrrrrrrrr : {server}");
    let driver = WebDriver::new(server, caps).await;

    match driver {
        Ok(driver_) => {
            driver_.goto("https://www.google.com").await;
        }
        Err(_) => {
            println!("error")
        }
    };
}

fn pick_a_free_port() -> Option<Port> {
    let ip4 = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    Some(TcpListener::bind(ip4).ok()?.local_addr().ok()?.port())
}
