use pest::Parser;
use::pest_derive::Parser;

#[derive(Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result< () > {
    let got = Grammar::parse(Rule::record, "-2345.54,-15")?;
    println!("{:#?}", got);
    Ok(())
}
