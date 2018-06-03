use mixture_lambda::ast::MixtureLambdaAST;
use pure_lambda::ast::PureLambdaAST;

impl MixtureLambdaAST {
    pub fn to_pure_ast(&self) -> PureLambdaAST {
        match self {
            MixtureLambdaAST::Id(id) => PureLambdaAST::Id(id.to_string()),
            MixtureLambdaAST::Apply(left, right) => {
                PureLambdaAST::Apply(Box::new(left.to_pure_ast()), Box::new(right.to_pure_ast()))
            }
            MixtureLambdaAST::Def(id, body) => {
                PureLambdaAST::Def(id.to_string(), Box::new(body.to_pure_ast()))
            }
            MixtureLambdaAST::S => PureLambdaAST::Def(
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
            MixtureLambdaAST::K => PureLambdaAST::Def(
                "x".to_string(),
                Box::new(PureLambdaAST::Def(
                    "y".to_string(),
                    Box::new(PureLambdaAST::Id("x".to_string())),
                )),
            ),
            MixtureLambdaAST::I => PureLambdaAST::Def(
                "x".to_string(),
                Box::new(PureLambdaAST::Id("x".to_string())),
            ),
        }
    }
}
