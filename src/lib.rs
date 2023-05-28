pub mod lints;

use lints::snake_case_names_lint::SnakeCaseNamesLint;
pub use lsp_types::{Diagnostic, DiagnosticSeverity, Range};
use tan::{ann::Ann, api::parse_string_all, error::Error, expr::Expr, range::Ranged};

pub trait Lint {
    /// A unique name for the lint.
    fn name(&self) -> String;
    // #TODO needs return type.
    /// Runs after the parsing pass.
    fn run(&mut self, exprs: &[Ann<Expr>]);
}

pub fn compute_parse_error_diagnostics(input: &str, errors: Vec<Ranged<Error>>) -> Vec<Diagnostic> {
    let mut diagnostics: Vec<Diagnostic> = Vec::new();

    for error in errors {
        let start = tan::range::Position::from(error.1.start, input);
        let start = lsp_types::Position {
            line: start.line as u32,
            character: start.col as u32,
        };
        let end = tan::range::Position::from(error.1.end, input);
        let end = lsp_types::Position {
            line: end.line as u32,
            character: end.col as u32,
        };

        diagnostics.push(Diagnostic {
            range: Range { start, end },
            severity: None,
            code: None,
            code_description: None,
            source: None,
            message: error.0.to_string(),
            related_information: None,
            tags: None,
            data: None,
        });
    }

    diagnostics
}

pub fn compute_diagnostics(input: &str) -> Vec<Diagnostic> {
    let result = parse_string_all(input);

    // #TODO should run all lints.

    let diagnostics = match result {
        Ok(exprs) => {
            let mut diagnostics = Vec::new();

            let mut lint = SnakeCaseNamesLint::new(input);
            lint.run(&exprs);
            diagnostics.append(&mut lint.diagnostics);

            diagnostics
        }
        Err(errors) => compute_parse_error_diagnostics(input, errors),
    };

    diagnostics
}
