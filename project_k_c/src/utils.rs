use std::path::Path;

pub fn get_parent(path: &String) -> String {
    Path::new(path)
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// problem is with only "xx.ll" type of paths
// "./xx.ll" will work fine
pub fn correct_the_file_path(path: &String) -> String {
    let parent = get_parent(path);
    if parent.len() > 0 {
        parent + "/"
    } else {
        "./".to_string()
    }
}
