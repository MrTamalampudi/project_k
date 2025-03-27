use std::path::Path;

pub fn get_parent(path: &String) -> String {
    Path::new(path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}
