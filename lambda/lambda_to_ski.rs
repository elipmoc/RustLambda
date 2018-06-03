use lambda_ast::LambdaAST;

impl LambdaAST {
    pub fn to_ski_ast(&self) -> LambdaAST {
        match self {
            LambdaAST::Id(id) => one_rule(id),
            LambdaAST::Apply(left, right) => two_rule(left, right),
            LambdaAST::Def(id, body) => three_four_five_six_rules(&id, &body),
            _ => self.clone(),
        }
    }
    //Defで定義される束縛変数が使われているかを判定する
    //もし変数がシャドウイングされたら、そこで計算を終了する
    fn def_id_used(&self, id: &str) -> bool {
        match self {
            LambdaAST::Def(ref id2, _) if id2 == id => false,
            LambdaAST::Def(_, body) => body.def_id_used(&id),
            LambdaAST::Apply(left, right) => left.def_id_used(&id) || right.def_id_used(&id),
            LambdaAST::Id(id2) => id == id2,
            _ => false,
        }
    }
}

// T[x] => x
fn one_rule(id: &str) -> LambdaAST {
    LambdaAST::Id(id.to_string())
}

//T[(E₁ E₂)] => (T[E₁] T[E₂])
fn two_rule(left: &LambdaAST, right: &LambdaAST) -> LambdaAST {
    LambdaAST::Apply(Box::new(left.to_ski_ast()), Box::new(right.to_ski_ast()))
}

fn three_four_five_six_rules(id: &str, body: &LambdaAST) -> LambdaAST {
    match body {
        // T[λx.E] => (K T[E]) (if x does not occur free in E)
        _ if body.def_id_used(&id) == false => {
            LambdaAST::Apply(Box::new(LambdaAST::K), Box::new(body.to_ski_ast()))
        }
        // T[λx.x] => I
        LambdaAST::Id(id2) if id == id2 => LambdaAST::I,
        //T[λx.λy.E] => T[λx.T[λy.E]] (if x occurs free in E)
        LambdaAST::Def(_, _) => {
            LambdaAST::Def(id.to_string(), Box::new(body.to_ski_ast())).to_ski_ast()
        }
        //T[λx.(E₁ E₂)] => (S T[λx.E₁] T[λx.E₂])
        LambdaAST::Apply(left, right) => {
            let left = LambdaAST::Def(id.to_string(), left.clone()).to_ski_ast();
            let right = LambdaAST::Def(id.to_string(), right.clone()).to_ski_ast();
            let apply1 = LambdaAST::Apply(Box::new(LambdaAST::S), Box::new(left));
            LambdaAST::Apply(Box::new(apply1), Box::new(right))
        }
        _ => panic!("error!"),
    }
}
