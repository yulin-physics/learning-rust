fn main() {
    let a = remove_at_most_one("hell e ? h");
    println!("{}", a);
}

fn word(s: &str) -> bool {
    let s = s.to_lowercase();
    for (i, c) in s.chars().enumerate() {
        if i == s.len() / 2 {
            return true;
        } else if c != s.chars().nth(s.len() - i - 1).unwrap() {
            return false;
        }
    }
    true
}

fn sentence(s: &str) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    word(&s)
}

fn remove_at_most_one(s: &str) -> bool {
    let s = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>();
    let (left_removed, right_removed) = remove(&s);
    word(&left_removed) || word(&right_removed)
}

fn remove(s: &str) -> (String, String) {
    let s = s.to_lowercase();
    for (i, c) in s.chars().enumerate() {
        if i == s.len() / 2 {
            break;
        } else if c != s.chars().nth(s.len() - i - 1).unwrap() {
            let mut left_removed = s.to_string().clone();
            let mut right_removed = s.to_string().clone();
            left_removed.remove(i);
            right_removed.remove(s.len() - i - 1);
            return (left_removed, right_removed);
        }
    }
    ("".to_string(), "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_words() {
        for (example, expected) in [
            ("level", true),
            ("racecar", true),
            ("notapalindrome", false),
        ]
        .iter()
        {
            assert_eq!(word(*example), *expected);
        }
    }

    #[test]
    fn test_palindrome_sentence() {
        for (example, expected) in [
            ("Do geese see God?", true),
            ("Was it a car or a cat I saw?", true),
            ("aaaaaaaa??????????????", true),
            ("Waasdcs it a car or a cat I saw?", false),
        ]
        .iter()
        {
            assert_eq!(sentence(*example), *expected);
        }
    }

    #[test]
    fn test_palindrome_remove_one() {
        for (example, expected) in [
            ("aba", true),
            ("Dod geese see God?", true),
            ("Was I it a car or a cat I saw??", true),
            ("Wastt I it a car or a cat I saw??", false),
            ("Wasd I it a car or a cat I fsaw??", false),
        ]
        .iter()
        {
            assert_eq!(remove_at_most_one(*example), *expected)
        }
    }
}
