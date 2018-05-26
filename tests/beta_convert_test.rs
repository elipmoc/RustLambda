extern crate lambda;

use lambda::lambda_parser::lambda_parse;

fn beta_convert_eq(from: &str, to: &str) {
    match lambda_parse(from) {
        Ok((ast, _)) => assert_eq!(ast.beta_convert().show(), to),
        Err(_) => assert!(false),
    }
}

#[test]
fn beta_convert_test() {
    beta_convert_eq("λx.x", "(λx.x)");
    beta_convert_eq("λx.λy.x y", "(λx.(λy.(x y)))");
    beta_convert_eq("(λx.x) k", "k");
}
