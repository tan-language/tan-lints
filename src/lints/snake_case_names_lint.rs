use lsp_types::{Diagnostic, DiagnosticSeverity};
use tan::expr::Expr;

use crate::Lint;

// #todo find better name
/// The suggested maximum length for symbols.
const MAX_NAME_LENGTH: usize = 42;

pub struct SnakeCaseNamesLint {
    pub diagnostics: Vec<Diagnostic>,
}

impl Lint for SnakeCaseNamesLint {
    fn name(&self) -> String {
        "snake_case_names".to_owned()
    }

    fn run(&mut self, exprs: &[Expr]) {
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

    fn run_expr(&mut self, expr: &Expr) {
        if let Expr::List(terms) = expr.unpack() {
            if terms.is_empty() {
                return;
            }

            let head = &terms[0];

            match head.unpack() {
                Expr::Symbol(s) if s == "let" => {
                    if terms.len() < 2 {
                        // #todo this is an error, report!
                        return;
                    }

                    let name = &terms[1];

                    let Expr::Symbol(s) = name.unpack() else {
                        // #todo this is an error, report!
                        return;
                    };

                    if s.len() > MAX_NAME_LENGTH {
                        if let Some(range) = name.range() {
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
                    // #todo move above!
                    for t in terms {
                        self.run_expr(t);
                    }
                }
            }
        }
    }
}

impl Default for SnakeCaseNamesLint {
    fn default() -> Self {
        Self::new()
    }
}
