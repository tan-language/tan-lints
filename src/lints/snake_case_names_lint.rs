use lsp_types::{Diagnostic, DiagnosticSeverity};
use tan::{ann::Ann, expr::Expr};

use crate::Lint;

// #TODO find better name
/// The suggested maximum length for symbols.
const MAX_NAME_LENGTH: usize = 42;

pub struct SnakeCaseNamesLint {
    pub diagnostics: Vec<Diagnostic>,
}

impl Lint for SnakeCaseNamesLint {
    fn name(&self) -> String {
        "snake_case_names".to_owned()
    }

    fn run(&mut self, exprs: &[Ann<Expr>]) {
        for expr in exprs {
            self.run_expr(expr);
        }
    }
}

impl SnakeCaseNamesLint {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    fn run_expr(&mut self, expr: &Ann<Expr>) {
        if let Ann(Expr::List(terms), _) = expr {
            if terms.len() < 1 {
                return;
            }

            let head = &terms[0];

            match head {
                Ann(Expr::Symbol(s), _) if s == "let" => {
                    if terms.len() < 2 {
                        // #TODO this is an error, report!
                        return;
                    }

                    let name = &terms[1];

                    let Ann(Expr::Symbol(s), _) = name else {
                        // #TODO this is an error, report!
                        return;
                    };

                    if s.len() > MAX_NAME_LENGTH {
                        if let Some(range) = name.get_range() {
                            let start = lsp_types::Position {
                                line: range.start.line as u32,
                                character: range.start.col as u32,
                            };
                            let end = lsp_types::Position {
                                line: range.end.line as u32,
                                character: range.end.col as u32,
                            };

                            self.diagnostics.push(Diagnostic {
                                range: lsp_types::Range { start, end },
                                severity: Some(DiagnosticSeverity::WARNING),
                                code: None,
                                code_description: None,
                                source: None,
                                message: format!("The symbol `{s}` is too long."),
                                related_information: None,
                                tags: None,
                                data: None,
                            });
                        }
                    }
                }
                _ => {
                    // #TODO move above!
                    for t in terms {
                        self.run_expr(t);
                    }
                }
            }
        }
    }
}
