use crate::args::Arguments;
use rand::Rng;
use rand::seq::SliceRandom;

const LOWERCASE_LETTERS : [char; 26] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
const UPPERCASE_LETTERS : [char; 26] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'];
const DIGITS : [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const PUNCTUATIONS : [char; 12] = ['.', '*', '?', '#', '\'', ',', '+', '/', '!', '^', '%', '&'];

pub fn generate_password(arguments: &Arguments) -> String {

    let mut password = String::new();
    let len = rand::thread_rng().gen_range(arguments.min_length..=arguments.max_length);
    let characters : Vec<char>;

    if arguments.no_digit && arguments.no_punc {
        characters = LOWERCASE_LETTERS.iter().copied().chain(UPPERCASE_LETTERS.iter().copied()).collect();
    } else if arguments.no_digit {
        characters = LOWERCASE_LETTERS.iter().copied()
            .chain(UPPERCASE_LETTERS.iter().copied())
            .chain(PUNCTUATIONS.iter().copied())
            .collect();
    } else if arguments.no_punc {
        characters = LOWERCASE_LETTERS.iter().copied()
            .chain(UPPERCASE_LETTERS.iter().copied())
            .chain(DIGITS.iter().copied())
            .collect();
    } else {
        characters = LOWERCASE_LETTERS.iter().copied()
            .chain(UPPERCASE_LETTERS.iter().copied())
            .chain(DIGITS.iter().copied())
            .chain(PUNCTUATIONS.iter().copied())
            .collect();
    }

    for _ in 0..len {
        password.push(select_random_char(&characters))
    }

    return password
}

fn select_random_char(chars: &Vec<char>) -> char {
    return *chars.choose(&mut rand::thread_rng()).unwrap();
}
