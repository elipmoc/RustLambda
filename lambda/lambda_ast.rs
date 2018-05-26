//ラムダ計算のAST
#[derive(Debug, Clone)]
pub enum LambdaAST {
    Def(String, Box<LambdaAST>),
    Apply(Box<LambdaAST>, Box<LambdaAST>),
    Id(String),
}

impl LambdaAST {
    //ASTが表しているラムダ計算の文字列を返す
    pub fn show(&self) -> String {
        match self {
            LambdaAST::Def(arg_name, body) => "(".to_string() + &show_def(arg_name, body) + ")",
            LambdaAST::Apply(left, right) => "(".to_string() + &show_apply(&left, &right) + ")",
            LambdaAST::Id(name) => name.to_string(),
        }
    }
}

fn show_def(arg_name: &str, body: &Box<LambdaAST>) -> String {
    "λ".to_string() + arg_name + "." + &body.show()
}

fn show_apply(left: &Box<LambdaAST>, right: &Box<LambdaAST>) -> String {
    left.show() + " " + &right.show()
}
