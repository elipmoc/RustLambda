use mixture_lambda::ast::MixtureLambdaAST;
use pure_lambda::ast::PureLambdaAST;

pub trait LambdaAST {
    fn show(&self) -> String;
    fn to_pure(&self) -> PureLambdaAST;
    fn to_ski(&self) -> MixtureLambdaAST;
    fn reduction(&self) -> PureLambdaAST {
        self.to_pure().beta_convert()
    }
}

pub trait LambdaASTConvert<From: LambdaAST> {
    fn convert(&From) -> Self;
}
