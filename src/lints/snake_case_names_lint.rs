use tan::{ann::Ann, expr::Expr};

use crate::Lint;

pub struct SnakeCaseNamesLint {}

impl Lint for SnakeCaseNamesLint {
    fn name() -> String {
        "snake_case_names".to_owned()
    }

    fn run(_expr: &Ann<Expr>) {
        todo!()
    }
}
