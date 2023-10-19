use std::process::exit;

fn main() {
    let input_strings = ["apple", "first"];
    for input_string in input_strings {
        let output_string = pig_latin(input_string);
        println!("{input_string} -> {output_string}");
    }
}

fn pig_latin(input_string: &str) -> String {
    // Get first character
    let first_char = input_string.chars().next();
    let first_char = match first_char {
        Option::Some(c) => c,
        Option::None => exit(-1),
    };

    // Check if first character a vowel
    let vowels = ['a', 'i', 'u', 'e', 'o'];
    let mut is_vowel = false;
    for vowel in vowels {
        if first_char == vowel {
            is_vowel = true;
            break;
        }
    }

    let mut output_string = input_string.to_string();
    // Remove first character if it's not a vowel
    if !is_vowel {
        output_string.remove(0);
    }
    // Add ay or hay
    output_string = if !is_vowel {
        format!("{output_string}-{first_char}ay")
    } else {
        format!("{output_string}-hay")
    };
    output_string
}
