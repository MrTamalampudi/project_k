use engine::execute;
use miette::GraphicalTheme;
use miette::LabeledSpan;
use miette::Result;
use miette::ThemeStyles;
use owo_colors::style;
use parser::parse;
use parser::CompilationContext;
use std::env;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<()> {
    log4rs::init_file("./log_config.yml", Default::default()).unwrap();
    env::set_var("RUST_BACKTRACE", "1");
    let argss: Vec<String> = env::args().collect();
    miette_config();
    let source_path = match argss.get(1) {
        Some(path) => path,
        None => {
            let report = miette::miette!(
                severity = miette::Severity::Error,
                "Please provide a file path",
            );
            return Err(report);
        }
    };

    let source = match fs::read_to_string(source_path) {
        Ok(string) => string,
        Err(error) => {
            let report = miette::miette!(severity = miette::Severity::Error, "{}", error);
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
            severity = miette::Severity::Error,
            code = "Parsing Error",
            help = "Please check documentation for clarity",
            labels = errors,
            url = "https://github.com",
            "Error while parsing the test case"
        )
        .with_source_code(source);
        return Err(report);
    }
    Ok(())
}

fn miette_config() {
    miette::set_hook(Box::new(|_| {
        let theme_style = ThemeStyles {
            error: style().red(),
            warning: style().yellow(),
            advice: style().cyan(),
            help: style().cyan(),
            link: style().cyan().underline().bold(),
            linum: style().dimmed(),
            highlights: vec![style().yellow().bold()],
        };
        let mut graphical_theme = GraphicalTheme::unicode();
        graphical_theme.styles = theme_style;
        Box::new(
            miette::MietteHandlerOpts::new()
                .terminal_links(true)
                .unicode(true)
                .context_lines(2)
                .break_words(true)
                .graphical_theme(graphical_theme)
                .build(),
        )
    }))
    .unwrap();
}
