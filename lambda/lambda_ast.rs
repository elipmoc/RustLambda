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
            LambdaAST::Def(arg_name, body) => show_def(arg_name, body),
            LambdaAST::Apply(left, right) => show_apply(&left, &right),
            LambdaAST::Id(name) => name.to_string(),
        }
    }
}

fn show_def(arg_name: &str, body: &Box<LambdaAST>) -> String {
    "λ".to_string() + arg_name + "." + &body.show()
}

fn show_apply(left: &Box<LambdaAST>, right: &Box<LambdaAST>) -> String {
    let left_show = match **left {
        LambdaAST::Def(_, _) => "(".to_string() + &left.show() + ")",
        _ => left.show(),
    };
    let right_show = match **right {
        LambdaAST::Apply(_, _) => "(".to_string() + &right.show() + ")",
        _ => right.show(),
    };
    left_show + " " + &right_show
}
