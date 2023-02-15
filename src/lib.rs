pub mod lints;

use tan::{ann::Ann, expr::Expr};

pub trait Lint {
    /// A unique name for the lint.
    fn name(&self) -> String;
    // #TODO needs return type.
    /// Runs after the parsing pass.
    fn run(&mut self, exprs: &[Ann<Expr>]);
}
