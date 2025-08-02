use project_k::compile;
use project_k::CompilationContext;
use std::env;
use std::path::PathBuf;

fn main() {
    log4rs::init_file("./log_config.yml", Default::default()).unwrap();
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => panic!("please provide a file path"),
    };

    let mut ctx = CompilationContext::new(PathBuf::from(source_path.clone()));
    compile(&mut ctx);
    // println!("{:#?}", ctx.errors.errors);
    // println!("{:#?}", ctx.program);
}
