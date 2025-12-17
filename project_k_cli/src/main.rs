use project_k::parse;
use project_k::CompilationContext;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    log4rs::init_file("./log_config.yml", Default::default()).unwrap();
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => {
            eprintln!("please provide a file path");
            return;
        }
    };

    let source = match fs::read_to_string(source_path) {
        Ok(string) => string,
        Err(error) => {
            eprintln!("{:#?}", error);
            return;
        }
    };

    let mut ctx = CompilationContext::new(PathBuf::from(source_path.clone()));
    parse(source, &mut ctx);
    // println!("{:#?}", ctx.errors.errors);
    // println!("{:#?}", ctx.program);
}
