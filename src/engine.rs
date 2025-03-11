use crate::actions::Action;
use crate::ast::{TestStep, Testcase};
use thirtyfour::prelude::*;

#[tokio::main]
pub async fn execute_test_case(test_case: Testcase) {
    let caps = DesiredCapabilities::chrome();
    let driver_result = WebDriver::new("http://localhost:33917", caps).await;

    let driver = match driver_result {
        Ok(result) => result,
        Err(result) => panic!("there was an error{}", result),
    };

    let teststeps: &Vec<TestStep> = test_case.get_teststeps();

    for teststep in teststeps.iter() {
        let action = teststep.action.clone();
        match action {
            Action::NAVIGATE => execute_navigate_action(&driver, teststep).await,
            Action::CLICK => execute_click_action(&driver, teststep).await,
            Action::BACK => execute_back_action(&driver, teststep).await,
            Action::FORWARD => execute_forward_action(&driver, teststep).await,
            Action::NONE => todo!(),
            _ => break,
        }
    }
}

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
