use engine::execute;
use miette::LabeledSpan;
use miette::Result;
use miette::SourceSpan;
use parser::parse;
use parser::CompilationContext;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    log4rs::init_file("./log_config.yml", Default::default()).unwrap();
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => {
            let report = miette::miette!(
                // Those fields are optional
                severity = miette::Severity::Error,
                code = "expected::rparen",
                url = "https://example.com",
                // Rest of the arguments are passed to `format!`
                // to form diagnostic message
                "Please provide a file path",
            );
            return Err(report);
        }
    };

    let source = match fs::read_to_string(source_path) {
        Ok(string) => string,
        Err(error) => {
            let report = miette::miette!(
                // Those fields are optional
                severity = miette::Severity::Error,
                code = "expected::rparen",
                url = "https://example.com",
                // Rest of the arguments are passed to `format!`
                // to form diagnostic message
                "Please provide a file path",
            );
            return Err(report);
        }
    };

    let mut ctx = CompilationContext::new(PathBuf::from(source_path.clone()));
    parse(source.as_str(), &mut ctx);
    if ctx.errors.errors.is_empty() {
        execute(&mut ctx.ast.testcase);
    } else {
        let errors: Vec<_> = ctx
            .errors
            .errors
            .iter()
            .cloned()
            .map(|e| LabeledSpan::at(e.span, e.message))
            .collect();
        let report = miette::miette!(
            // Those fields are optional
            severity = miette::Severity::Error,
            code = "Parsing Error",
            help = "Please check documentation for clarity",
            labels = errors,
            url = "https://github.com",
            // Rest of the arguments are passed to `format!`
            // to form diagnostic message
            "Error while parsing the test case"
        )
        .with_source_code(source);
        return Err(report);
    }
    println!("variables {:#?}", ctx.ast.testcase.variables);
    Ok(())
}
