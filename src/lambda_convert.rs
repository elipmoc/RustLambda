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

    //ASTのマッチするidを別のASTで置き換える
    fn replace_id(&self, id: &str, replace_ast: &LambdaAST) -> LambdaAST {
        match self {
            LambdaAST::Id(x) if x == id => replace_ast.clone(),
            LambdaAST::Id(_) => self.clone(),
            LambdaAST::Def(param, body) => LambdaAST::Def(
                param.to_string(),
                Box::new(body.replace_id(id, replace_ast)),
            ),
            LambdaAST::Apply(left, right) => {
                let left = left.replace_id(id, replace_ast);
                let right = right.replace_id(id, replace_ast);
                LambdaAST::Apply(Box::new(left), Box::new(right))
            }
        }
    }
}
