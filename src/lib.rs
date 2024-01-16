pub mod lints;

use lints::{
    no_arrow_in_names_lint::NoArrowInNamesLint, snake_case_names_lint::SnakeCaseNamesLint,
};
pub use lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tan::{api::parse_string_all, error::Error, expr::Expr};

pub trait Lint {
    /// A unique name for the lint.
    fn name(&self) -> String;
    // #todo needs return type.
    /// Runs after the parsing pass.
    fn run(&mut self, exprs: &[Expr]);
}

pub fn compute_parse_error_diagnostics(errors: Vec<Error>) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    for error in errors {
        if let Some(range) = error.range() {
            let start = lsp_types::Position {
                line: range.start.line as u32,
                character: range.start.col as u32,
            };
            let end = lsp_types::Position {
                line: range.end.line as u32,
                character: range.end.col as u32,
            };

            diagnostics.push(Diagnostic {
                range: Range { start, end },
                severity: None,
                code: None,
                code_description: None,
                source: None,
                message: error.to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        } else {
            // #todo how to handle errors without range?
        }
    }

    diagnostics
}

pub fn compute_diagnostics(input: &str) -> Vec<Diagnostic> {
    let result = parse_string_all(input);

    // #todo should run all lints.

    match result {
        Ok(exprs) => {
            let mut diagnostics = Vec::new();

            // #todo some Lints may need the input!
            // #todo how to loop over lints?

            let mut lint = SnakeCaseNamesLint::new();
            lint.run(&exprs);
            diagnostics.append(&mut lint.diagnostics);

            let mut lint = NoArrowInNamesLint::new();
            lint.run(&exprs);
            diagnostics.append(&mut lint.diagnostics);

            diagnostics
        }
        Err(errors) => compute_parse_error_diagnostics(errors),
    }
}
