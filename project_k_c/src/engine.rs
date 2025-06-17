use std::cell::RefCell;
use std::rc::Rc;

use crate::ast::testcase::TestCase;
use crate::ast::teststep::TestStep;
use crate::enums::{Browser, CapabilityValue};
use crate::keywords::TokenType;
use thirtyfour::prelude::*;

#[tokio::main]
pub async fn execute_test_case(test_case: Rc<RefCell<TestCase>>) {
    let caps = DesiredCapabilities::chrome();
    let testcase = test_case.borrow_mut();
    let driver_url = get_driver_url(&testcase);
    let driver_result = WebDriver::new(driver_url, caps).await;

    let driver = match driver_result {
        Ok(result) => result,
        Err(result) => panic!("there was an error{}", result),
    };

    let teststeps: &Vec<TestStep> = testcase.get_teststeps();

    // execute_teststeps(&driver, teststeps).await;
}

fn get_driver_url(test_case: &TestCase) -> String {
    let result = test_case.get_capability(
        &TokenType::CAPS(crate::enums::Capabilities::DRIVERURL)
            .to_string()
            .to_string(),
    );
    match result {
        CapabilityValue::STRING(string) => string,
        _ => panic!("Expecting Driver url"),
    }
}

fn get_browser_capabilites(test_case: TestCase) -> Browser {
    let browser = test_case.get_capability(
        &TokenType::CAPS(crate::enums::Capabilities::BROWSER)
            .to_string()
            .to_string(),
    );
    match browser {
        CapabilityValue::BROWSER(browser) => browser,
        _ => panic!("Expecting Browser capability value"),
    }
}

// async fn execute_teststeps(driver: &WebDriver, teststeps: &Vec<TestStep>) {
//     for teststep in teststeps.iter() {
//         let action = teststep.class.clone();
//         match action {
//             Action::NAVIGATE => execute_navigate_action(&driver, teststep).await,
//             Action::CLICK => execute_click_action(&driver, teststep).await,
//             Action::BACK => execute_back_action(&driver, teststep).await,
//             Action::FORWARD => execute_forward_action(&driver, teststep).await,
//             Action::NONE => todo!(),
//             _ => break,
//         }
//     }
// }

async fn execute_back_action(driver: &WebDriver, teststep: &TestStep) {
    match driver.back().await {
        Ok(_) => println!("Navigate back"),
        Err(error) => eprintln!("{}", error),
    }
}

async fn execute_forward_action(driver: &WebDriver, teststep: &TestStep) {
    match driver.forward().await {
        Ok(_) => println!("Navigate forward"),
        Err(error) => eprintln!("{}", error),
    }
}

async fn execute_navigate_action(driver: &WebDriver, teststep: &TestStep) {
    let url = teststep.arguments.get(0).map(|x| x.as_str());
    match url {
        Some(url) => match driver.goto(url).await {
            Ok(_) => println!("Navigated to {}", url),
            Err(error) => eprintln!("{}", error),
        },
        None => {}
    }
}

async fn execute_click_action(driver: &WebDriver, teststep: &TestStep) {
    let locator = teststep.arguments.get(0).map(|x| x.as_str());
    match locator {
        Some(locator) => {
            match driver.find(By::XPath(locator)).await {
                Ok(element) => match element.click().await {
                    Ok(_) => println!("click action perfomed on {}", element),
                    Err(error) => eprintln!("{}", error),
                },
                Err(element) => eprintln!("{}", element),
            };
        }
        None => {}
    }
}
