use lambda_trait::LambdaAST;

//純粋なラムダ計算のAST
#[derive(Debug, Clone)]
pub enum PureLambdaAST {
    Def(String, Box<PureLambdaAST>),
    Apply(Box<PureLambdaAST>, Box<PureLambdaAST>),
    Id(String),
}

impl LambdaAST for PureLambdaAST {
    fn show(&self) -> String {
        self.to_lambda_ast().show()
    }

    fn to_pure(&self) -> PureLambdaAST {
        self.clone()
    }
}
