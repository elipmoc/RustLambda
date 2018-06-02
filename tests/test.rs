extern crate lambda;

use lambda::lambda_parser::lambda_parse;
use lambda::ski_ast::SkiAST;

fn beta_convert_eq(from: &str, to: &str) {
    match lambda_parse(from) {
        Ok((ast, _)) => assert_eq!(ast.beta_convert().show(), to),
        Err(_) => assert!(false),
    }
}

#[test]
fn beta_convert_test() {
    beta_convert_eq("λx.x", "λx.x");
    beta_convert_eq("λx.λy.x y", "λx.λy.x y");
    beta_convert_eq("(λx.x) k", "k");
    beta_convert_eq("λx.λy.λz.x z (y z)", "λx.λy.λz.x z (y z)");
    beta_convert_eq("(λx.λy.λz.x z (y z)) a b c", "a c (b c)");
    beta_convert_eq("(λx.x) λz.λy.z", "λz.λy.z");
    beta_convert_eq(
        "(λa.λb.λf.λx.a f (b f x)) (λf.λx.f x) (λf.λx.f (f x))",
        "λf.λx.f (f (f x))",
    );
}

#[test]
fn ski_ast_show_test() {
    assert_eq!(SkiAST::I.show(), "I");
    assert_eq!(
        SkiAST::Apply(Box::new(SkiAST::S), Box::new(SkiAST::K)).show(),
        "S K"
    );
    assert_eq!(
        SkiAST::Apply(
            Box::new(SkiAST::S),
            Box::new(SkiAST::Apply(Box::new(SkiAST::I), Box::new(SkiAST::S)))
        ).show(),
        "S (I S)"
    );
}
