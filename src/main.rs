extern crate lambda;
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
                let beta_convert = value.to_pure_ast().beta_convert().to_lambda_ast();
                let to_ski = beta_convert.to_ski_ast();
                let to_pure = to_ski.to_pure_ast();
                let beta_convert2 = to_pure.beta_convert().to_lambda_ast();
                println!("beta convert: {}", beta_convert.show());
                println!("to_ski: {}", to_ski.show());
                println!("to_pure: {}", to_pure.to_lambda_ast().show());
                println!("beta convert2: {}", beta_convert2.show());
            }
            Err(_) => (),
        };
    }
}
