#[macro_use]
extern crate combine;
mod lambda_ast;
mod lambda_convert;
mod lambda_parser;
use lambda_parser::lambda_parse;
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
        println!("ParseResult: {:?}", expr);
        match expr {
            //パースしたASTが示すラムダ計算を表示
            Ok((value, _)) => {
                println!("show: {}", value.show());
                println!("beta convert: {}", value.beta_convert().show());
            }
            Err(_) => (),
        };
    }
}
