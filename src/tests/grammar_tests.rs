use anyhow::Result;
use pest::Parser;
use pest_derive::Parser;

#[test]
fn file_test_single_record() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::file, "-273.45,12\n")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(got.as_str(), "-273.45,12\n");

    Ok(())
}

#[test]
fn file_test_multiple_records() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::file, "-273.45,12\n-273.45,12\n")?
        .next()
        .ok_or_else(|| anyhow!("No pair"))?;
    assert_eq!(got.as_str(), "-273.45,12\n-273.45,12\n");

    Ok(())
}

#[test]
fn file_test_invalid_records() -> anyhow::Result<()> {
    let got = Grammar::parse(Rule::file, "-273.45 12");
    assert!(got.is_err());

    let got = Grammar::parse(Rule::file, "-273.45,12");
    assert!(got.is_err());

    Ok(())
}
