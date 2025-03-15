#[derive(Debug, Clone)]
#[allow(non_camel_case_types, unused)]
pub enum Action {
    NAVIGATE,
    CLICK,
    UPLOAD,
    BACK,
    FORWARD,
    NONE,
}

#[derive(Debug, Clone)]
#[allow(non_camel_case_types, unused)]
pub enum ActionOption {
    UNIQUE_EMAIL,
    TO,
    NONE,
}
