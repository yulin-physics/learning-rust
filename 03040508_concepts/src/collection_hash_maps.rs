use std::collections::HashMap;

pub fn run() {
    // empty hashmap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 30);
    // overwriting a value
    scores.insert(String::from("Blue"), 25);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // can construct hashmap using iterators and the collect method on a vector of tuples
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 30];
    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores).collect();

    let team_name = String::from("Blue");
    let get_score = scores.get(&team_name);
    match get_score {
        Some(score) => println!("Score: {}", score),
        None => println!("doesn't exist"),
    }

    // only insert a value if the key has no value
    scores.entry(String::from("blue")).or_insert(50);
    scores.entry(String::from("purple")).or_insert(0);

    println!("Score: {:?}", scores);
    println!("Count: {:?}", count_word("hello hello how are you"));
}

fn count_word(s: &str) -> HashMap<&str, u32> {
    let mut map = HashMap::new();
    for w in s.split_whitespace() {
        let count = map.entry(w).or_insert(0);
        *count += 1;
    }
    map
}
