use lsp_types::{Diagnostic, DiagnosticSeverity};
use tan::expr::Expr;

use crate::Lint;

// #
pub struct NoArrowInNamesLint {
    pub diagnostics: Vec<Diagnostic>,
}

impl Lint for NoArrowInNamesLint {
    fn name(&self) -> String {
        "no_arrow_in_names".to_owned()
    }

    fn run(&mut self, exprs: &[Expr]) {
        for expr in exprs {
            self.run_expr(expr);
        }
    }
}

impl NoArrowInNamesLint {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    // #todo weird name.
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

                    // #todo extract some machinery

                    if s.contains("->") {
                        // #todo #fix the range seems wrong, up to the `->` start, weird.
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
                                message: "The use of `->` should be avoided in names".to_string(),
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

impl Default for NoArrowInNamesLint {
    fn default() -> Self {
        Self::new()
    }
}
