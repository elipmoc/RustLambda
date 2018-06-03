use lambda_ast::LambdaAST;
use pure_lambda_ast::PureLambdaAST;

impl LambdaAST {
    pub fn to_pure_ast(&self) -> PureLambdaAST {
        match self {
            LambdaAST::Id(id) => PureLambdaAST::Id(id.to_string()),
            LambdaAST::Apply(left, right) => {
                PureLambdaAST::Apply(Box::new(left.to_pure_ast()), Box::new(right.to_pure_ast()))
            }
            LambdaAST::Def(id, body) => {
                PureLambdaAST::Def(id.to_string(), Box::new(body.to_pure_ast()))
            }
            LambdaAST::S => PureLambdaAST::Def(
                "x".to_string(),
                Box::new(PureLambdaAST::Def(
                    "y".to_string(),
                    Box::new(PureLambdaAST::Def(
                        "z".to_string(),
                        Box::new(PureLambdaAST::Apply(
                            Box::new(PureLambdaAST::Apply(
                                Box::new(PureLambdaAST::Id("x".to_string())),
                                Box::new(PureLambdaAST::Id("z".to_string())),
                            )),
                            Box::new(PureLambdaAST::Apply(
                                Box::new(PureLambdaAST::Id("y".to_string())),
                                Box::new(PureLambdaAST::Id("z".to_string())),
                            )),
                        )),
                    )),
                )),
            ),
            LambdaAST::K => PureLambdaAST::Def(
                "x".to_string(),
                Box::new(PureLambdaAST::Def(
                    "y".to_string(),
                    Box::new(PureLambdaAST::Id("x".to_string())),
                )),
            ),
            LambdaAST::I => PureLambdaAST::Def(
                "x".to_string(),
                Box::new(PureLambdaAST::Id("x".to_string())),
            ),
        }
    }
}
