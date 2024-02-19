// src/lib/parser.rs

// dependences
use nom::bytes::complete::tag;
use nom::character::complete::alpha0;
use nom::multi::separated_list0;
use nom::IResult;

// a parser which will find any sequence of alphabetic characters: a-z, A-Z
fn parse_word(input: &str) -> IResult<&str, &str> {
    alpha0(input)
}

// a parsser which will recognize a white space
fn parse_space(input: &str) -> IResult<&str, &str> {
    tag(" ")(input)
}

// a combinator which will take a sentence as input and convert it into a vector of all the individual words
pub fn parse_sentence(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list0(parse_space, parse_word)(input)
}

// unit tests for each parser and combinator
#[cfg(test)]

mod tests {

    use super::*;

    #[test]
    fn test_parse_word() {
        let input = "test";
        let result = parse_word(input);
        assert_eq!(result, Ok(("", "test")));
    }

    #[test]
    fn test_parse_space() {
        let input = " test";
        let result = parse_space(input);
        assert_eq!(result, Ok(("test", " ")));
    }

    #[test]
    fn test_parse_sentence() {
        let input = "The quick brown fox jumped over the lazy dog";
        let result = parse_sentence(input);
        assert_eq!(
            result,
            Ok((
                "",
                vec!("The", "quick", "brown", "fox", "jumped", "over", "the", "lazy", "dog")
            ))
        );
    }
}
