pub mod errors;
pub mod token;
pub mod lexer;
pub mod nfa;
pub mod parser;
pub mod matcher;

pub use crate::{
    lexer::Lexer,
    errors::ParseError,
    parser::Parser,
    matcher::Matcher,
};

pub struct RRegex { 
    matcher: Matcher
}

impl RRegex {
    pub fn new(regex: String) -> Result<Self, ParseError> {
        let lexer = Lexer::new(regex);
        let mut parser = Parser::new(lexer);
        let nfa = parser.parse()?;
        let matcher = Matcher::new(nfa);

        Ok(RRegex { matcher })
    }

    pub fn matches(&self, input: &str) -> bool {
        self.matcher.matches(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_matches(s1: &str, s2: &str) {
        let s1 = s1.to_string();
        let s2 = s2.to_string();
        let expected = regex::Regex::new(&format!("^({})$", &s1)[..]).unwrap().is_match(&s2);

        let rregex = RRegex::new(s1.clone())
            .expect(&format!("Failed to create regex ({s1}). Regex parsed {expected}.")[..]);

        assert_eq!(
            rregex.matches(&s2),
            expected,
            "Test failed for regex: '{}', input: '{}'",
            s1,
            s2
        );
    }

    #[test]
    fn test_union() {
        let test_regex = "a|b";
        let test_strings = ["a", "b", "c"];
        let expected_strings = [true, true, false];

        let regex = RRegex::new(test_regex.to_string()).unwrap();

        for (result, expected) in test_strings.iter().zip(expected_strings.iter()) {
            let result = regex.matches(*result);
            assert_eq!(result, *expected);
        }
    }

    #[test]
    fn test_regex_literals() {
        // Positive test cases
        test_matches("h", "h"); // Exact match

        // Negative test cases
        test_matches("a", "b"); // Different literals
        test_matches("a", "ab"); // Different literals

        // Edge cases
        test_matches("a", ""); // Non-empty regex, empty input
    }

    #[test]
    fn test_regex_union() {
        // Positive test cases
        test_matches("a|b", "a"); // "a" matches the first alternative
        test_matches("a|b", "b"); // "b" matches the second alternative
        test_matches("a|b", "a|b"); // Edge case: literal match of the regex itself. rust returns true because of how were doing full text search by appending

        // Negative test cases
        test_matches("a|b", "c"); // "c" does not match either "a" or "b"
        test_matches("a|b", ""); // Empty input does not match either alternative
        test_matches("a|b", "ab"); // "ab" is not a valid match for "a|b"

        // Edge cases
        test_matches("a|", "a"); // Union with an empty alternative
        test_matches("a|", ""); // Empty alternative matches an empty string
        test_matches("|a", "b"); // Empty alternative does not match non-empty input
        test_matches("a|a", "a"); // Duplicate alternatives should still match
    }

    #[test]
    fn test_parentheses() {
        // Positive test cases
        test_matches("(a)", "a");          // Parentheses around a single literal
        test_matches("(ab)", "ab");        // Parentheses around a concatenation
        test_matches("(a|b)", "a");        // Union inside parentheses matches "a"
        test_matches("(a|b)", "b");        // Union inside parentheses matches "b"
        test_matches("a(b)", "ab");        // Concatenation with parentheses
        test_matches("(a)(b)", "ab");      // Concatenation of two parenthesized groups
        test_matches("(a|b)c", "ac");      // Union followed by a literal
        test_matches("(a|b)c", "bc");      // Union followed by a literal
        test_matches("a(b|c)", "ab");      // Literal followed by a union
        test_matches("a(b|c)", "ac");      // Literal followed by a union

        // Negative test cases
        test_matches("(a)", "b");          // Parentheses around "a" do not match "b"
        test_matches("(ab)", "a");         // Parentheses around "ab" do not match partial input
        test_matches("(a|b)", "c");        // Union inside parentheses does not match "c"
        test_matches("a(b)", "a");         // Concatenation with parentheses requires both parts
        test_matches("(a|b)c", "a");       // Requires "c" after the union
        test_matches("a(b|c)", "a");       // Requires either "b" or "c" after "a"

        // Edge cases
        test_matches("()", "");           // Empty parentheses match the empty string
        test_matches("(a|)", "a");         // Union with an empty alternative matches "a"
        test_matches("(a|)", "");          // Union with an empty alternative matches ""
        test_matches("(|a)", "a");         // Union with an empty alternative matches "a"
        test_matches("(|a)", "");          // Union with an empty alternative matches ""
    }

    // #[test]
    // fn test_regex_concatenation() {
    //     // Positive test cases
    //     test_matches("ab", "ab"); // "ab" matches the concatenation of "a" and "b"
    //     test_matches("abc", "abc"); // Longer concatenation: "abc" matches exactly
    //     test_matches("a|bc", "bc"); // Union with concatenation: "bc" matches the second alternative

    //     // Negative test cases
    //     test_matches("ab", "a"); // "a" is incomplete; does not match "ab"
    //     test_matches("ab", "b"); // "b" is incomplete; does not match "ab"
    //     test_matches("ab", "ba"); // "ba" is out of order; does not match "ab"
    //     test_matches("ab", ""); // Empty input does not match "ab"

    //     // Edge cases
    //     test_matches("a", "a"); // Single character concatenation (trivial case)
    //     test_matches("", ""); // Empty pattern matches empty input
    //     test_matches("a", ""); // Non-empty pattern does not match empty input
    //     test_matches("a*b", "aab"); // Concatenation with repetition: "a*b" matches "aab"
    //     test_matches("a*b", "b"); // Concatenation with zero repetitions: "a*b" matches "b"
    // }
}