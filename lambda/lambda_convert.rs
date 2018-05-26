use lambda_ast::LambdaAST;

impl LambdaAST {
    //β簡約する
    pub fn beta_convert(&self) -> LambdaAST {
        match self {
            LambdaAST::Id(_) => self.clone(),
            LambdaAST::Def(_, _) => self.clone(),
            LambdaAST::Apply(left, right) => {
                let left = left.beta_convert();
                let right = right.beta_convert();
                match left {
                    LambdaAST::Id(_) => LambdaAST::Apply(Box::new(left), Box::new(right)),
                    LambdaAST::Apply(_, _) => LambdaAST::Apply(Box::new(left), Box::new(right)),
                    LambdaAST::Def(param, body) => body.replace_id(&param, &right),
                }
            }
        }
    }

    fn alpha_convert(&self) -> LambdaAST {
        match self {
            LambdaAST::Def(param, body) => {
                let new_param = param.to_string() + "`";
                let body = body.alpha_convert_replace_id(param);
                LambdaAST::Def(new_param, Box::new(body))
            }
            _ => self.clone(),
        }
    }

    fn alpha_convert_replace_id(&self, id: &str) -> LambdaAST {
        match self {
            LambdaAST::Def(param, _) if param == id => self.clone(),
            LambdaAST::Def(param, body) => {
                let body = body.alpha_convert_replace_id(id);
                LambdaAST::Def(param.clone(), Box::new(body))
            }
            LambdaAST::Apply(left, right) => {
                let left = left.alpha_convert_replace_id(id);
                let right = right.alpha_convert_replace_id(id);
                LambdaAST::Apply(Box::new(left), Box::new(right))
            }
            LambdaAST::Id(x) if x == id => LambdaAST::Id(x.to_string() + "`"),
            LambdaAST::Id(_) => self.clone(),
        }
    }

    //ASTのマッチするidを別のASTで置き換える
    fn replace_id(&self, id: &str, replace_ast: &LambdaAST) -> LambdaAST {
        match self {
            LambdaAST::Id(x) if x == id => replace_ast.clone(),
            LambdaAST::Id(_) => self.clone(),
            LambdaAST::Def(param, body) if param == id => self.clone(),
            LambdaAST::Def(param, body) => match replace_ast {
                LambdaAST::Id(x) if param == x => self.alpha_convert().replace_id(id, replace_ast),
                _ => LambdaAST::Def(
                    param.to_string(),
                    Box::new(body.replace_id(id, replace_ast)),
                ),
            },
            LambdaAST::Apply(left, right) => {
                let left = left.replace_id(id, replace_ast);
                let right = right.replace_id(id, replace_ast);
                LambdaAST::Apply(Box::new(left), Box::new(right))
            }
        }
    }
}
