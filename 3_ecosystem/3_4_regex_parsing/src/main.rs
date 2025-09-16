use std::fmt::Error;

use once_cell::sync::Lazy;
use regex::Regex;

// nom crate-based implementation
use nom::{
    branch::alt,
    bytes::{take_while, take_while1, tag},
    character::complete::char,
    combinator::{map, opt, value},
    error::ErrorKind,
    IResult, Parser,
};

fn parse_sign_nom(input: &str) -> IResult<&str, Option<Sign>> {
    let mut parser = opt(alt((
        map(char('+'), |_| Sign::Plus),
        map(char('-'), |_| Sign::Minus),
    )));
    parser.parse(input)
}

// Тестирование
fn main() {
    let test_sign_1 = [
        ("+", Some(Sign::Plus)),
        ("-", Some(Sign::Minus)),
        ("", None),
        ("abc", None),
    ];

    for (input, expected) in test_sign_1 {
        let result = parse_sign_nom(input);
        println!("Input: '{}'", input);
        match result {
            Ok((remaining, sign)) => {
                println!("Success: {:?}, Remaining: '{}'", sign, remaining);
                assert_eq!(sign, expected);
            }
            Err(e) => {
                println!("Error: {:?}", e);
                assert_eq!(None, expected);
            }
        }
        println!("---");
    }

}

// Regex-based implementation

// To omit unnecessary performance penalty we should compile regular expression once and reuse its compilation result
static REGEX_SIGN: Lazy<Regex> = Lazy::new(|| Regex::new("[+-]").unwrap()); // compiles once on the first use

impl From<regex::Match<'_>> for Sign {
    fn from(value: regex::Match<'_>) -> Self {
        let res = value.as_str();
        match res {
            "+" => Sign::Plus,
            "-" => Sign::Minus,
            _ => panic!("something went wrong"),
        }
    }
}

static REGEX_PRECISION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\.(?:(?:\d+|\w+)\$|\*|\d+)").unwrap()); // compiles once on the first use

fn is_precision(input: &str) -> bool {
    REGEX_PRECISION.is_match(input)
}

static REGEX_WIDTH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^|[\s<^>+#-])(\d+)(?:$|[.\]])").unwrap()); // compiles once on the first use

fn is_width(input: &str) -> bool {
    REGEX_WIDTH.is_match(input)
}

fn parse_sign(input: &str) -> Option<Sign> {
    if let Some(mat) = REGEX_SIGN.find(input) {
        Some(mat.into())
    } else {
        None
    }
}

fn parse_width(input: &str) -> Option<usize> {
    if let Some(caps) = REGEX_WIDTH.captures(input) {
        if let Some(width_match) = caps.get(1) {
            return width_match.as_str().parse::<usize>().ok();
        }
    }

    None
}

fn parse_precision(input: &str) -> Option<Precision> {
    if let Some(mat) = REGEX_PRECISION.find(input) {
        let intermed = mat.as_str();
        match intermed {
            // If Precision is Asterisk
            ".*" => Some(Precision::Asterisk),

            // If Precision is Argument
            s if s.ends_with('$') => {
                let arg_no_dollar = s.trim_end_matches('$');
                let arg = arg_no_dollar.trim_start_matches('.');
                if let Ok(num) = arg.parse::<usize>() {
                    Some(Precision::Argument(num))
                } else {
                    None //Some(Precision::Argument(arg.parse::<usize>))
                }
            }

            // If Precision is Integer
            s => {
                let arg = s.trim_start_matches('.');
                if let Ok(num) = arg.parse::<usize>() {
                    Some(Precision::Integer(num))
                } else {
                    None
                }
            }
            _ => {
                println!("THIS PRECISION IS UNCATCHED: {:?}", mat.as_str());
                None
            }
        }
    } else {
        None
    }
}

fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let parsed_sign = parse_sign(input);
    let parsed_width = parse_width(input);
    let parsed_precision = parse_precision(input);
    (parsed_sign, parsed_width, parsed_precision)
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq)]
enum Precision {
    Integer(usize),
    Argument(usize),
    Asterisk,
}

#[cfg(test)]
mod spec {
    use super::*;

    #[test]
    fn parses_sign() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", None),
            (">+8.*", Some(Sign::Plus)),
            ("-.1$x", Some(Sign::Minus)),
            ("a^#043.8?", None),
        ] {
            let (sign, ..) = parse(input);
            assert_eq!(sign, expected);
        }
    }

    #[test]
    fn parses_width() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(8)),
            (">+8.*", Some(8)),
            ("-.1$x", None),
            ("a^#043.8?", Some(43)),
        ] {
            let (_, width, _) = parse(input);
            assert_eq!(width, expected);
        }
    }

    #[test]
    fn parses_precision() {
        for (input, expected) in vec![
            ("", None),
            (">8.*", Some(Precision::Asterisk)),
            (">+8.*", Some(Precision::Asterisk)),
            ("-.1$x", Some(Precision::Argument(1))),
            ("a^#043.8?", Some(Precision::Integer(8))),
        ] {
            let (_, _, precision) = parse(input);
            assert_eq!(precision, expected);
        }
    }
}
