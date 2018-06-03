use combine;
use combine::char::{char, digit, letter};
use combine::parser::combinator::not_followed_by;
use combine::{many, value, Parser, Stream};
use mixture_lambda::ast::MixtureLambdaAST;
use std::result::Result;

/*
BNF
    <expr> := <apply>
    <apply>:= <term>{' '<term>}
    <def>  := 'λ'<id>'.'<expr>
    <term> := <id>|<paren>|<def>
    <id>   := 'λ' , '(' , ')' , ' ' を含まない任意文字列
    <paren>:= '(' <expr> ')'
 */

//ラムダ計算をパースする
pub fn lambda_parse(
    s: &str,
) -> Result<(MixtureLambdaAST, &str), combine::error::StringStreamError> {
    expr_parser().parse(s)
}

//<expr>
parser!{
   fn expr_parser[I]()(I) -> MixtureLambdaAST
    where [I: Stream<Item=char>]
    {
        apply_parser()
    }
}

//<apply>
parser!{
    fn apply_parser[I]()(I) -> MixtureLambdaAST
    where [I: Stream<Item=char>]
    {
        (term_parser(),many::<Vec<MixtureLambdaAST>,_>(char(' ').with(term_parser()))).then(
            |(first_expr,tail_exprs)|{
                let mut expr = first_expr;
                for tail_expr in tail_exprs {
                    expr=MixtureLambdaAST::Apply(Box::new(expr),Box::new(tail_expr))
                }
                value(expr)
            }
        )
    }
}

//<def>
parser!{
    fn def_parser[I]()(I) -> MixtureLambdaAST
    where [I: Stream<Item=char>]
    {
        char('λ').with( (id_parser(),char('.').with(expr_parser())) )
            .then(
                |(s,body)|value(MixtureLambdaAST::Def(s,Box::new(body)))
            )
    }
}

//<term>
parser!{
    fn term_parser[I]()(I) -> MixtureLambdaAST
    where [I: Stream<Item=char>]
    {
        id_parser().then( |s|value(MixtureLambdaAST::Id(s)) ).or(paren_parser()).or(def_parser())
    }
}

//<id>
parser!{
    fn id_parser[I]()(I) -> String
    where [I: Stream<Item=char>]
    {
        let head_id_parser = not_followed_by(char('λ').or(digit())).with(letter());
        (head_id_parser,many::<String, _>(letter().or(digit())))
            .then(|(head,tails):(char,String)| value(head.to_string()+&tails))
    }
}

//<paren>
parser!{
    fn paren_parser[I]()(I) -> MixtureLambdaAST
    where [I: Stream<Item=char>]
    {
        char('(').with(expr_parser()).skip(char(')'))
    }
}
