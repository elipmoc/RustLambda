//純粋なラムダ計算のAST
#[derive(Debug, Clone)]
pub enum PureLambdaAST {
    Def(String, Box<PureLambdaAST>),
    Apply(Box<PureLambdaAST>, Box<PureLambdaAST>),
    Id(String),
}
