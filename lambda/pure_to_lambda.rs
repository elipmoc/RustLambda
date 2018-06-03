use lambda_ast::LambdaAST;
use pure_lambda_ast::PureLambdaAST;

impl PureLambdaAST {
    pub fn to_lambda_ast(&self) -> LambdaAST {
        match self {
            PureLambdaAST::Id(id) => LambdaAST::Id(id.to_string()),
            PureLambdaAST::Def(id, body) => {
                LambdaAST::Def(id.to_string(), Box::new(body.to_lambda_ast()))
            }
            PureLambdaAST::Apply(left, right) => LambdaAST::Apply(
                Box::new(left.to_lambda_ast()),
                Box::new(right.to_lambda_ast()),
            ),
        }
    }
}
