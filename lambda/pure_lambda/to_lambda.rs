use mixture_lambda::ast::MixtureLambdaAST;
use pure_lambda::ast::PureLambdaAST;

impl PureLambdaAST {
    pub fn to_lambda_ast(&self) -> MixtureLambdaAST {
        match self {
            PureLambdaAST::Id(id) => MixtureLambdaAST::Id(id.to_string()),
            PureLambdaAST::Def(id, body) => {
                MixtureLambdaAST::Def(id.to_string(), Box::new(body.to_lambda_ast()))
            }
            PureLambdaAST::Apply(left, right) => MixtureLambdaAST::Apply(
                Box::new(left.to_lambda_ast()),
                Box::new(right.to_lambda_ast()),
            ),
        }
    }
}
