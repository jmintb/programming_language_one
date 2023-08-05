use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct LangParser;

fn main() {
    let input = "print(\"hello world\")".to_string();

    let parsed_input = LangParser::parse(Rule::call, &input).unwrap();

    println!("{parsed_input:#?}");
}
