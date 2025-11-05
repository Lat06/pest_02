use anyhow::Result;
use pest::Parser; // Цей імпорт ПОТРІБЕН тут для методу .parse()
use pest_02::{Grammar, Rule}; // Замініть 'pest_02' на назву вашого проєкту, якщо вона інша

fn main() -> Result<()> {
    let a = Grammar::parse(Rule::file, "-173.490,-15\n")?;
    println!("Результат з main: {:?}", a);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result, anyhow};

    // ВИПРАВЛЕНО: Залишили тільки 'Pair', оскільки 'Pairs' не використовується
    use pest::iterators::Pair;

    #[test]
    fn basic_test() -> Result<()> {
        // Тепер тестуємо поле, а не файл
        let got: Pair<'_, Rule> = Grammar::parse(Rule::field, "-273.45")?
            .next()
            .ok_or_else(|| anyhow!("No pair"))?;

        assert_eq!(got.as_str(), "-273.45");
        assert_eq!(got.as_span().start(), 0);
        assert_eq!(got.as_span().end(), 7);

        println!("Результат з basic_test: {:?}", got);
        Ok(())
    }
}
