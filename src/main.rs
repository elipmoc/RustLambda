#[macro_use]
extern crate combine;
use combine::char::{alpha_num, char, digit, letter, space};
use combine::parser::combinator::not_followed_by;
use combine::parser::function;
use combine::{many, sep_by, token, value, ParseResult, Parser, Positioned, Stream, StreamOnce,
              many1};
use std::io;
use std::iter::FromIterator;

#[derive(Debug, Clone)]
enum Expr {
    Def(String, Box<Expr>),
    Apply(Box<Expr>, Box<Expr>),
    Id(String),
}

fn show_expr(expr: &Expr) -> String {
    match expr {
        Expr::Def(arg_name, body) => "(".to_string() + &show_def(arg_name, body) + ")",
        Expr::Apply(left, right) => "(".to_string() + &show_apply(&left, &right) + ")",
        Expr::Id(name) => name.to_string(),
    }
}

fn show_def(arg_name: &str, body: &Box<Expr>) -> String {
    "λ".to_string() + arg_name + "." + &show_expr(body)
}

fn show_apply(left: &Box<Expr>, right: &Box<Expr>) -> String {
    show_expr(left) + " " + &show_expr(right)
}

/*
<expr> := <apply>
<apply>:= <term>{' '<term>}
<def>  := 'λ'<id>'.'<expr>
<term> := <id>|<paren>|<def>
<id>   := 'λ' , '(' , ')' , ' ' を含まない任意文字列
<paren>:= '(' <expr> ')'

 */
parser!{
    fn expr_parser[I]()(I) -> Expr
    where [I: Stream<Item=char>]
    {
        apply_parser()
    }
}

parser!{
    fn apply_parser[I]()(I) -> Expr
    where [I: Stream<Item=char>]
    {
        (term_parser(),many::<Vec<Expr>,_>(char(' ').with(term_parser()))).then(
            |(first_expr,tail_exprs)|{
                let mut expr = first_expr;
                for tail_expr in tail_exprs {
                    expr=Expr::Apply(Box::new(expr),Box::new(tail_expr))
                }
                value(expr)
            }
        )
    }
}

parser!{
    fn def_parser[I]()(I) -> Expr
    where [I: Stream<Item=char>]
    {
        char('λ').with( (id_parser(),char('.').with(expr_parser())) )
            .then(
                |(s,body)|value(Expr::Def(s,Box::new(body)))
            )
    }
}

parser!{
    fn term_parser[I]()(I) -> Expr
    where [I: Stream<Item=char>]
    {
        id_parser().then( |s|value(Expr::Id(s)) ).or(paren_parser()).or(def_parser())
    }
}

parser!{
    fn id_parser[I]()(I) -> String
    where [I: Stream<Item=char>]
    {
        let head_id_parser = not_followed_by(char('λ').or(digit())).with(letter());
        (head_id_parser,many::<String, _>(letter().or(digit())))
            .then(|(head,tails):(char,String)| value(head.to_string()+&tails))
    }
}

parser!{
    fn paren_parser[I]()(I) -> Expr
    where [I: Stream<Item=char>]
    {
        char('(').with(expr_parser()).skip(char(')'))
    }
}

fn expr_parse(s: &str) {
    println!("{:?}", expr_parser().parse(s));
    match expr_parser().parse(s) {
        Ok((value, _)) => println!("{}", show_expr(&value)),
        Err(_) => (),
    };
}

fn main() {
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let a: &str = &input;

        expr_parse(a);
    }
}
