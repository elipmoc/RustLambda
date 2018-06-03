extern crate lambda;
use lambda::lambda_trait::LambdaAST;
use lambda::mixture_lambda::parser::lambda_parse;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        println!("input: {}", input);
        let s: &str = &input;
        //パース
        let expr = lambda_parse(s);
        //パース結果を表示
        //println!("ParseResult: {:?}", expr);
        match expr {
            //パースしたASTが示すラムダ計算を表示
            Ok((value, _)) => {
                //  println!("show: {}", value.show());
                let beta_convert = value.reduction();
                let to_ski = beta_convert.to_mixture_lambda().to_ski();
                let to_pure = to_ski.to_pure();
                let beta_convert2 = to_pure.reduction();
                println!("beta convert: {}", beta_convert.show());
                println!("to_ski: {}", to_ski.show());
                println!("to_pure: {}", to_pure.show());
                println!("beta convert2: {}", beta_convert2.show());
            }
            Err(_) => (),
        };
    }
}
