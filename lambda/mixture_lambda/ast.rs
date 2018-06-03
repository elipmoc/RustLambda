use lambda_trait::LambdaAST;
use pure_lambda::ast::PureLambdaAST;

//ラムダ計算のAST
#[derive(Debug, Clone)]
pub enum MixtureLambdaAST {
    Def(String, Box<MixtureLambdaAST>),
    Apply(Box<MixtureLambdaAST>, Box<MixtureLambdaAST>),
    Id(String),
    S,
    K,
    I,
}

impl LambdaAST for MixtureLambdaAST {
    //ASTが表しているラムダ計算の文字列を返す
    fn show(&self) -> String {
        self.show()
    }

    fn to_pure(&self) -> PureLambdaAST {
        self.to_pure_ast()
    }

    fn to_mixture_lambda(&self) -> MixtureLambdaAST {
        self.clone()
    }
}
