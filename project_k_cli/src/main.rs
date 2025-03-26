use project_k::compile;
use project_k::CompilationContext;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => panic!("please provide a file path"),
    };
    let mut ctx = CompilationContext::new(source_path.clone());
    compile(source_path, &mut ctx);
    println!("{:#?}", ctx.program);
}
