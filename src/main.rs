#[macro_use]
extern crate combine;
mod lambda_ast;
mod lambda_parser;
use lambda_parser::lambda_parse;
use std::io;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let s: &str = &input;
        //パース
        let expr = lambda_parse(s);
        //パース結果を表示
        println!("{:?}", expr);
        match expr {
            //パースしたASTが示すラムダ計算を表示
            Ok((value, _)) => println!("{}", value.show()),
            Err(_) => (),
        };
    }
}
