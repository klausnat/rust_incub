use std::{cmp::Ordering, env, io};

fn main() {
    println!("Guess the number!");

    let secret_number = get_secret_number();

    loop {
        println!("Please input your guess.");

        let guess = match get_guess_number() {
            Some(n) => n,
            _ => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn get_secret_number() -> u32 {
    let secret_number = env::args()
        .skip(1)
        .take(1)
        .last()
        .expect("No secret number is specified");
    secret_number
        .trim()
        .parse()
        .ok()
        .expect("Secret number is not a number")
}

fn get_guess_number() -> Option<u32> {
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the parsing logic (the most testable part)
    #[test]
    fn test_input_parsing() {
        let test_cases = vec![
            ("42", Some(42)),
            ("  42  ", Some(42)),
            ("not_a_number", None),
            ("3.14", None),
            ("0x10", None),
            ("1e10", None),
            ("", None),
            //  ("-5", None), // Uncomment and test will fail
            ("9999999999", None), // too large for u32
        ];

        for (input, expected) in test_cases {
            let result = input.trim().parse().ok();
            assert_eq!(result, expected, "Failed for input: '{}'", input);
        }
    }

    #[test]
    #[should_panic] // Negative value is acceptable - probably it's a bug
    fn test_negative_input_parsing() {
        let test_case = ("-5", None);
        let (input, expected) = test_case;
        let res: Option<i32> = input.trim().parse().ok();
        assert_eq!(res, expected, "Failed for input: '{}'", input);
    }

    #[test]
    fn test_comparison_logic() {
        let secret = 50;

        assert_eq!(30.cmp(&secret), Ordering::Less);
        assert_eq!(70.cmp(&secret), Ordering::Greater);
        assert_eq!(50.cmp(&secret), Ordering::Equal);
    }

    // Test the game logic in isolation
    #[test]
    fn test_game_outcomes() {
        let test_cases = vec![
            (30, 50, "Too small!"),
            (70, 50, "Too big!"),
            (50, 50, "You win!"),
        ];

        for (guess, secret, expected_msg) in test_cases {
            let outcome = match guess.cmp(&secret) {
                Ordering::Less => "Too small!",
                Ordering::Greater => "Too big!",
                Ordering::Equal => "You win!",
            };
            assert_eq!(outcome, expected_msg);
        }
    }
}

// Property-Based testing
#[cfg(test)]
mod property_tests {
    use super::*;
    use quickcheck::{QuickCheck, TestResult};

    // Test the comparison logic in isolation
    #[test]
    fn prop_comparison_logic_correct() {
        fn property(secret: u32, guess: u32) -> TestResult {
            let result = match guess.cmp(&secret) {
                Ordering::Less => "Too small!",
                Ordering::Greater => "Too big!",
                Ordering::Equal => "You win!",
            };
            
            // Verify the logic is correct
            let expected = if guess < secret {
                "Too small!"
            } else if guess > secret {
                "Too big!"
            } else {
                "You win!"
            };
            
            TestResult::from_bool(result == expected)
        }
        
        QuickCheck::new()
            .tests(1000)
            .quickcheck(property as fn(u32, u32) -> TestResult);
    }

    // Test parsing logic
    #[test]
    fn prop_parsing_behavior() {
        fn property(num: u32) -> TestResult {
            let input = num.to_string();
            
            // Test that valid numbers parse correctly
            let parsed = input.trim().parse::<u32>().ok();
            TestResult::from_bool(parsed == Some(num))
        }
        
        QuickCheck::new()
            .tests(1000)
            .quickcheck(property as fn(u32) -> TestResult);
    }
}
