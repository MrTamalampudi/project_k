use std::path::{Path, PathBuf};

pub fn get_parent(path: &PathBuf) -> PathBuf {
    path.parent().unwrap().to_path_buf()
}

// problem is with only "xx.ll" type of paths
// "./xx.ll" will work fine
