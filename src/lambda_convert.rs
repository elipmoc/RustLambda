use lambda_ast::LambdaAST;

pub fn beta_convert(ast: &LambdaAST) -> LambdaAST {
    match ast {
        LambdaAST::Id(_) => ast.clone(),
        LambdaAST::Def(_, _) => ast.clone(),
        LambdaAST::Apply(left, right) => {
            let left = beta_convert(left);
            let right = beta_convert(right);
            match left {
                LambdaAST::Id(_) => LambdaAST::Apply(Box::new(left), Box::new(right)),
                LambdaAST::Apply(_, _) => LambdaAST::Apply(Box::new(left), Box::new(right)),
                LambdaAST::Def(param, body) => replace_id(&body, &param, &right),
            }
        }
    }
}

fn replace_id(ast: &LambdaAST, id: &str, replace_ast: &LambdaAST) -> LambdaAST {
    match ast {
        LambdaAST::Id(x) if x == id => replace_ast.clone(),
        LambdaAST::Id(_) => ast.clone(),
        LambdaAST::Def(param, body) => LambdaAST::Def(
            param.to_string(),
            Box::new(replace_id(body, id, replace_ast)),
        ),
        LambdaAST::Apply(left, right) => {
            let left = replace_id(left, id, replace_ast);
            let right = replace_id(right, id, replace_ast);
            LambdaAST::Apply(Box::new(left), Box::new(right))
        }
    }
}
