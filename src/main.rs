use rregex::RRegex;
use rregex::ParseError;
use std::env;

fn main() -> Result<(), ParseError>{
    let args:Vec<String> = env::args().collect();

    if args.len() > 1 { test_match(&args[1], args.get(2).unwrap())? }

    else {
        test_match("a", "a")?;
        test_match("a|b", "a")?;
        test_match("a|b", "b")?;
        test_match("a|b", "c")?;
        test_match("abc", "abc")?;
        test_match("abc", "abcdef")?;
        test_match("a*", "aaaaaaaaa")?;
    }

    Ok(())
}

fn test_match(regex: &str, input: &str) -> Result<(), ParseError>{
    let rregex = RRegex::new(regex.to_string())?;

    println!("Matching {} to input '{}'.", &regex, &input);

    if rregex.matches(input) {
        Ok(println!("Matches!"))
    }
    else {
        Ok(println!("No Match."))
    }
}