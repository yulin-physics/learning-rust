// Convert strings to pig latin.
// The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.”
// Words that start with a vowel have “hay” added to the end instead (“apple” becomes “apple-hay”).

pub enum Word {
    Vowel,
    Consonant,
}

pub fn convert(s: &str) -> String {
    match parse_type(s) {
        Word::Vowel => convert_vowel_word(s),
        Word::Consonant => convert_consonant_word(s),
    }
}

fn convert_vowel_word(s: &str) -> String {
    format!("{}-hay", s)
}

fn convert_consonant_word(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let (first_letter, rest_s) = chars.split_at(1);
    let first_letter: String = first_letter.into_iter().collect();
    let rest_s: String = rest_s.into_iter().collect();
    format!("{}-{}ay", rest_s, first_letter)
}

fn parse_type(s: &str) -> Word {
    let first_letter = s.chars().next().unwrap();
    let is_vowel = "aeiouAEIOU".contains(first_letter);
    if is_vowel {
        return Word::Vowel;
    } else {
        return Word::Consonant;
    }
}
