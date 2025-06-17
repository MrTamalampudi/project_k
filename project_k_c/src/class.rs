#![allow(non_camel_case_types, unused)]

#[derive(Debug, Clone)]
pub enum Class {
    //has authentication todo
    HAS_AUTHENTICATION,
    HAS_DOWNLOADS,
    NAVIGATION,
    OPTIONS,
    TARGET_LOCATOR,
    TIMEOUTS,
    ELEMENT,
    WINDOW,
    WEB_DRIVER,
    ALERT,
}

#[derive(Debug, Clone)]
pub enum Method {
    HAS_DOWNLOADS(Has_Downloads),
    NAVIGATION(Navigation),
    OPTIONS(Options),
    TARGET_LOCATOR(Target_Locator),
    TIMEOUTS(Timeouts),
    ELEMENT(Element),
    WINDOW(Window),
    WEB_DRIVER(WebDriver),
    ALERT(Alert),
}

#[derive(Debug, Clone)]
pub enum Has_Downloads {
    DELETE_DOWNLOADABLE_FILES,
    DOWNLOAD_FILE,
    GET_DOWNLOADABLE_FILES,
}

#[derive(Debug, Clone)]
pub enum Navigation {
    BACK,
    FORWARD,
    REFRESH,
}

#[derive(Debug, Clone)]
pub enum Options {
    ADD_COOKIE,
    DELETE_ALL_COOKIES,
    DELETE_COOKIE,
    GET_COOKIE,
}

#[derive(Debug, Clone)]
pub enum Target_Locator {
    ACTIVE_ELEMENT,
    ALERT,
    FRAME,
    NEW_WINDOW,
    PARENT_FRAME,
    WINDOW,
}

#[derive(Debug, Clone)]
pub enum Timeouts {
    GET_IMPLICIT_TIMEOUT,
    GET_PAGE_LOAD_TIMEOUT,
    SET_IMPLICIT_TIMEOUT,
    SET_PAGE_LOAD_TIMEOUT,
}

#[derive(Debug, Clone)]
pub enum Element {
    CLEAR,
    CLICK,
    GET_ACCESSBILE_NAME,
    GET_ARIA_ROLE,
    GET_ATTRIBUTE,
    GET_CSS_VALUE,
    GET_DOM_PROPERTY,
    GET_LOCATION,
    GET_SIZE,
    GET_TAG_NAME,
    GET_TEXT,
    IS_DISPLAYED,
    IS_ENABLED,
    IS_SELECTED,
    SENDKEYS,
    SUBMIT,
}

#[derive(Debug, Clone)]
pub enum Window {
    FULL_SCREEN,
    GET_POSITION,
    GET_SIZE,
    MAXIMIZE,
    MINIMIZE,
}

#[derive(Debug, Clone)]
pub enum WebDriver {
    GET,
    GET_TITLE,
    GET_CURRENT_URL,
    GET_PAGE_SOURCE,
    GET_WINDOW_HANDLE,
    CLOSE,
    FIND_ELEMENT,
    NAVIGATE,
}

#[derive(Debug, Clone)]
pub enum Alert {
    ACCEPT,
    DISMISS,
    GET_TEXT,
    SEND_KEYS,
}
