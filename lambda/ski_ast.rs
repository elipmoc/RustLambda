//SKIコンビネータのAST
#[derive(Debug, Clone)]
pub enum SkiAST {
    S,
    K,
    I,
    Apply(Box<SkiAST>, Box<SkiAST>),
}


impl SkiAST {
    //ASTが表しているラムダ計算の文字列を返す
    pub fn show(&self) -> String {
        match self {
            SkiAST::Apply(left, right) => show_apply(&left, &right),
            SkiAST::S => "S".to_string(),
            SkiAST::K => "K".to_string(),
            SkiAST::I => "I".to_string(),
        }
    }
}

fn show_apply(left: &Box<SkiAST>, right: &Box<SkiAST>) -> String {
    let left_show = left.show();
    let right_show = match **right {
        SkiAST::Apply(_, _) => "(".to_string() + &right.show() + ")",
        _ => right.show(),
    };
    left_show + " " + &right_show
}

