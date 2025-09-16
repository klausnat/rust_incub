use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n, take_while1},
    character::complete::{digit1, one_of},
    combinator::{map_res, opt, peek, recognize},
    sequence::{preceded, tuple},
};

fn parse(input: &str) -> (Option<Sign>, Option<usize>, Option<Precision>) {
    let sign = parse_sign(input).map(|(_, sign)| sign).ok();
    let width = parse_width(input).map(|(_, width)| width).ok().flatten();
    let precision = parse_precision(input).map(|(_, prec)| prec).ok().flatten();

    (sign, width, precision)
}

fn parse_sign(input: &str) -> IResult<&str, Sign> {
    let (input, _) = skip_until_sign(input)?;
    let (input, sign_char) = one_of("+-")(input)?;

    let sign = match sign_char {
        '+' => Sign::Plus,
        '-' => Sign::Minus,
        _ => unreachable!(),
    };

    Ok((input, sign))
}

fn skip_until_sign(input: &str) -> IResult<&str, ()> {
    // Use take_while (without 1) to allow zero characters
    let (input, _) = take_while(|c: char| !"+-".contains(c) && !c.is_whitespace())(input)?;
    Ok((input, ()))
}

fn parse_width(input: &str) -> IResult<&str, Option<usize>> {
    // Взять только цифры, отбросить лишнее до цифр
    let (input, _) = take_while(|c: char| !c.is_ascii_digit() && "<^>+#-".contains(c))(input)?;

    // отбросить все, что после точки (т.к после точки начинается Precision)
    let (_, input) = take_while(|c: char| !".".contains(c))(input)?;

    // Now look for width digits with explicit type annotation
    let (input, width_str) = match digit1::<_, nom::error::Error<&str>>(input) {
        Ok((input, digits)) => {
            println!("INSIDE OK");
            (input, digits)
        }
        Err(_) => return Ok((input, None)),
    };

    // Check if these digits are actually a width (not part of something else)
    if input.starts_with('$') || input.starts_with('.') {
        Ok((input, None)) // This is probably a precision parameter, not width
    } else if let Ok(width) = width_str.parse() {
        Ok((input, Some(width)))
    } else {
        Ok((input, None))
    }
}
fn parse_precision(input: &str) -> IResult<&str, Option<Precision>> {
    // Look for the '.' prefix
    let (input, _) = take_while(|c: char| c != '.')(input)?;
    let (input, _) = tag(".")(input)?;

    // Parse what comes after the dot
    let result = alt((
        // Asterisk case
        map_tag("*", Precision::Asterisk),
        // Parameter case (ends with $)
        map_parameter,
        // Integer case
        map_integer_precision,
    ))
    .parse(input);

    match result {
        Ok((input, precision)) => Ok((input, Some(precision))),
        Err(_) => Ok((input, None)),
    }
}

fn map_tag<'a, O>(tag_str: &'static str, value: O) -> impl Fn(&'a str) -> IResult<&'a str, O>
where
    O: Clone,
{
    move |input| {
        let (input, _) = tag(tag_str)(input)?;
        Ok((input, value.clone()))
    }
}

fn map_parameter(input: &str) -> IResult<&str, Precision> {
    let (input, num_str) = digit1(input)?;
    let (input, _) = tag("$")(input)?;

    if let Ok(arg_num) = num_str.parse() {
        Ok((input, Precision::Argument(arg_num)))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )))
    }
}

fn map_integer_precision(input: &str) -> IResult<&str, Precision> {
    let (input, num_str) = digit1(input)?;

    if let Ok(precision) = num_str.parse() {
        Ok((input, Precision::Integer(precision)))
    } else {
        Err(nom::Err::Error(nom::error::Error::new(
            input,
            nom::error::ErrorKind::Digit,
        )))
    }
}

#[derive(Debug, PartialEq)]
enum Sign {
    Plus,
    Minus,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
            (">+9.*", Some(9)),
            ("-.1$x", None),
            //("a^#043.8?", Some(43)),
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
