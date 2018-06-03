use mixture_lambda::ast::MixtureLambdaAST;
use pure_lambda::ast::PureLambdaAST;

pub trait LambdaAST {
    fn show(&self) -> String;
    fn to_pure(&self) -> PureLambdaAST;
    fn to_mixture_lambda(&self) -> MixtureLambdaAST {
        self.to_pure().to_lambda_ast()
    }
    fn to_ski(&self) -> MixtureLambdaAST {
        self.to_mixture_lambda().to_ski_ast()
    }
    fn reduction(&self) -> PureLambdaAST {
        self.to_pure().beta_convert()
    }
}
