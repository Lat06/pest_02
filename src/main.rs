use pest::Parser;

#[derive(pest_derive::Parser)]
#[grammar = "./grammar.pest"]
pub struct Grammar;

fn main() -> anyhow::Result<()> {
    let a = Grammar::parse(Rule::file, "-173.490,-15\n")?;
    println!("{:?}", a);
    Ok(())
}
