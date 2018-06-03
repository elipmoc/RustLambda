use mixture_lambda::ast::MixtureLambdaAST;

impl MixtureLambdaAST {
    pub fn to_ski_ast(&self) -> MixtureLambdaAST {
        match self {
            MixtureLambdaAST::Id(id) => one_rule(id),
            MixtureLambdaAST::Apply(left, right) => two_rule(left, right),
            MixtureLambdaAST::Def(id, body) => three_four_five_six_rules(&id, &body),
            _ => self.clone(),
        }
    }
    //Defで定義される束縛変数が使われているかを判定する
    //もし変数がシャドウイングされたら、そこで計算を終了する
    fn def_id_used(&self, id: &str) -> bool {
        match self {
            MixtureLambdaAST::Def(ref id2, _) if id2 == id => false,
            MixtureLambdaAST::Def(_, body) => body.def_id_used(&id),
            MixtureLambdaAST::Apply(left, right) => left.def_id_used(&id) || right.def_id_used(&id),
            MixtureLambdaAST::Id(id2) => id == id2,
            _ => false,
        }
    }
}

// T[x] => x
fn one_rule(id: &str) -> MixtureLambdaAST {
    MixtureLambdaAST::Id(id.to_string())
}

//T[(E₁ E₂)] => (T[E₁] T[E₂])
fn two_rule(left: &MixtureLambdaAST, right: &MixtureLambdaAST) -> MixtureLambdaAST {
    MixtureLambdaAST::Apply(Box::new(left.to_ski_ast()), Box::new(right.to_ski_ast()))
}

fn three_four_five_six_rules(id: &str, body: &MixtureLambdaAST) -> MixtureLambdaAST {
    match body {
        // T[λx.E] => (K T[E]) (if x does not occur free in E)
        _ if body.def_id_used(&id) == false => {
            MixtureLambdaAST::Apply(Box::new(MixtureLambdaAST::K), Box::new(body.to_ski_ast()))
        }
        // T[λx.x] => I
        MixtureLambdaAST::Id(id2) if id == id2 => MixtureLambdaAST::I,
        //T[λx.λy.E] => T[λx.T[λy.E]] (if x occurs free in E)
        MixtureLambdaAST::Def(_, _) => {
            MixtureLambdaAST::Def(id.to_string(), Box::new(body.to_ski_ast())).to_ski_ast()
        }
        //T[λx.(E₁ E₂)] => (S T[λx.E₁] T[λx.E₂])
        MixtureLambdaAST::Apply(left, right) => {
            let left = MixtureLambdaAST::Def(id.to_string(), left.clone()).to_ski_ast();
            let right = MixtureLambdaAST::Def(id.to_string(), right.clone()).to_ski_ast();
            let apply1 = MixtureLambdaAST::Apply(Box::new(MixtureLambdaAST::S), Box::new(left));
            MixtureLambdaAST::Apply(Box::new(apply1), Box::new(right))
        }
        _ => panic!("error!"),
    }
}
