use mixture_lambda::ast::MixtureLambdaAST;

impl MixtureLambdaAST {
    //ASTが表しているラムダ計算の文字列を返す
    pub fn show(&self) -> String {
        match self {
            MixtureLambdaAST::Def(arg_name, body) => show_def(arg_name, body),
            MixtureLambdaAST::Apply(left, right) => show_apply(&left, &right),
            MixtureLambdaAST::Id(name) => name.to_string(),
            MixtureLambdaAST::S => "S".to_string(),
            MixtureLambdaAST::K => "K".to_string(),
            MixtureLambdaAST::I => "I".to_string(),
        }
    }
}

fn show_def(arg_name: &str, body: &Box<MixtureLambdaAST>) -> String {
    "λ".to_string() + arg_name + "." + &body.show()
}

fn show_apply(left: &Box<MixtureLambdaAST>, right: &Box<MixtureLambdaAST>) -> String {
    let left_show = match **left {
        MixtureLambdaAST::Def(_, _) => "(".to_string() + &left.show() + ")",
        _ => left.show(),
    };
    let right_show = match **right {
        MixtureLambdaAST::Apply(_, _) => "(".to_string() + &right.show() + ")",
        _ => right.show(),
    };
    left_show + " " + &right_show
}
