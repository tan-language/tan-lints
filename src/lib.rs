pub mod lints;

use tan::{ann::Ann, expr::Expr};

pub trait Lint {
    fn name() -> String;
    fn run(expr: &Ann<Expr>);
}
