#![allow(non_camel_case_types)]

use thirtyfour::By;

const CLASS: &'static str = "class:";
const CSS: &'static str = "css:";
const ID: &'static str = "id:";
const NAME: &'static str = "name:";
const TAG: &'static str = "tag:";

#[derive(Debug, Clone)]
pub enum LocatorStrategy {
    CLASSNAME(String),
    CSS_SELECTOR(String),
    ID(String),
    NAME(String),
    TAG_NAME(String),
    XPATH(String),
}

impl LocatorStrategy {
    pub fn parse(locator: &String) -> LocatorStrategy {
        let mut locator_ = locator.clone();
        if locator.starts_with(CLASS) {
            return LocatorStrategy::CLASSNAME(locator_.split_off(6));
        } else if locator.starts_with(CSS) {
            return LocatorStrategy::CSS_SELECTOR(locator_.split_off(4));
        } else if locator.starts_with(ID) {
            return LocatorStrategy::ID(locator_.split_off(3));
        } else if locator.starts_with(NAME) {
            return LocatorStrategy::NAME(locator_.split_off(5));
        } else if locator.starts_with(TAG) {
            return LocatorStrategy::TAG_NAME(locator_.split_off(4));
        } else if locator.starts_with("//") || locator.starts_with("/") {
            return LocatorStrategy::XPATH(locator_.clone());
        } else {
            return LocatorStrategy::XPATH(format!("//*[text()=\"{}\"", locator_));
        }
    }

    pub fn to_by(&self) -> By {
        match self {
            LocatorStrategy::CLASSNAME(classname) => By::ClassName(classname),
            LocatorStrategy::CSS_SELECTOR(css) => By::Css(css),
            LocatorStrategy::ID(id) => By::Id(id),
            LocatorStrategy::TAG_NAME(tag) => By::Tag(tag),
            LocatorStrategy::XPATH(xpath) => By::XPath(xpath),
            LocatorStrategy::NAME(name) => By::Name(name),
        }
    }
}
