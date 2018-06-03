use pure_lambda::ast::PureLambdaAST;

impl PureLambdaAST {
    //β簡約する
    pub fn beta_convert(&self) -> PureLambdaAST {
        match self {
            PureLambdaAST::Id(_) => self.clone(),
            PureLambdaAST::Def(param, body) => {
                PureLambdaAST::Def(param.to_string(), Box::new(body.beta_convert()))
            }
            PureLambdaAST::Apply(left, right) => {
                let left = left.beta_convert();
                let right = right.beta_convert();
                match left {
                    PureLambdaAST::Id(_) => PureLambdaAST::Apply(Box::new(left), Box::new(right)),
                    PureLambdaAST::Apply(_, _) => {
                        PureLambdaAST::Apply(Box::new(left), Box::new(right))
                    }
                    PureLambdaAST::Def(param, body) => {
                        body.replace_id(&param, &right).beta_convert()
                    }
                }
            }
        }
    }

    //ASTのマッチするidを別のASTで置き換える
    fn replace_id(&self, id: &str, replace_ast: &PureLambdaAST) -> PureLambdaAST {
        match self {
            PureLambdaAST::Id(x) if x == id => replace_ast.clone(),
            PureLambdaAST::Id(_) => self.clone(),
            PureLambdaAST::Def(param, _) if param == id => self.clone(),
            PureLambdaAST::Def(param, body) => {
                if replace_ast.def_id_used(param) {
                    self.alpha_convert().replace_id(id, replace_ast)
                } else {
                    PureLambdaAST::Def(
                        param.to_string(),
                        Box::new(body.replace_id(id, replace_ast)),
                    )
                }
            }
            PureLambdaAST::Apply(left, right) => {
                let left = left.replace_id(id, replace_ast);
                let right = right.replace_id(id, replace_ast);
                PureLambdaAST::Apply(Box::new(left), Box::new(right))
            }
        }
    }

    //Defで定義される束縛変数が使われているかを判定する
    //もし変数がシャドウイングされたら、そこで計算を終了する
    fn def_id_used(&self, id: &str) -> bool {
        match self {
            PureLambdaAST::Def(ref id2, _) if id2 == id => false,
            PureLambdaAST::Def(_, body) => body.def_id_used(&id),
            PureLambdaAST::Apply(left, right) => left.def_id_used(&id) || right.def_id_used(&id),
            PureLambdaAST::Id(id2) => id == id2,
        }
    }

    //関数をα変換する
    //λx.x → λx`.x`
    fn alpha_convert(&self) -> PureLambdaAST {
        match self {
            PureLambdaAST::Def(param, body) => {
                let new_param = param.to_string() + "`";
                let body = body.alpha_convert_replace_id(param);
                PureLambdaAST::Def(new_param, Box::new(body))
            }
            _ => self.clone(),
        }
    }

    fn alpha_convert_replace_id(&self, id: &str) -> PureLambdaAST {
        match self {
            PureLambdaAST::Def(param, _) if param == id => self.clone(),
            PureLambdaAST::Def(param, body) => {
                let body = body.alpha_convert_replace_id(id);
                PureLambdaAST::Def(param.clone(), Box::new(body))
            }
            PureLambdaAST::Apply(left, right) => {
                let left = left.alpha_convert_replace_id(id);
                let right = right.alpha_convert_replace_id(id);
                PureLambdaAST::Apply(Box::new(left), Box::new(right))
            }
            PureLambdaAST::Id(x) if x == id => PureLambdaAST::Id(x.to_string() + "`"),
            PureLambdaAST::Id(_) => self.clone(),
        }
    }
}
