use std::collections::HashMap;

use crate::logger;

pub fn hash_maps() {
    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team = String::from("Black");
    let score = 0;
    scores.insert(team, score);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let mut map: HashMap<String, i32> = HashMap::new();
    let team = String::from("Black");
    let score = 0;
    map.insert(team, score);

    // logger::info(&format!("team: {}, score: {}", team, score)); // キーと値は所有されるのでコンパイルエラーになる

    let team = String::from("Black");
    let score = 0;
    scores.insert(&team, &score);

    logger::info(&format!("team: {}, score: {}", team, score));

    let mut scores: HashMap<String, u32> = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let blue_team_score = scores.get(&String::from("Blue"));
    match blue_team_score {
        Some(score) => logger::info(&format!("Blue team score: {}", score)),
        None => logger::info("Blue team score: None"),
    }

    let black_team_score = scores.get(&String::from("Black"));
    match black_team_score {
        Some(score) => logger::info(&format!("Black team score: {}", score)),
        None => logger::info("Black team score: None"),
    }
    let team = String::from("Black");
    let score = 0;
    scores.insert(team, score);

    let black_team_score = scores.get(&String::from("Black"));
    match black_team_score {
        Some(score) => logger::info(&format!("Black team score: {}", score)),
        None => logger::info("Black team score: None"),
    }

    for (key, value) in &scores {
        logger::info(&format!("{}: {}", key, value));
    }

    let team = String::from("Black");
    let score = 100;
    scores.insert(team, score);

    let black_team_score = scores.get(&String::from("Black"));
    match black_team_score {
        Some(score) => logger::info(&format!("Black team score: {}", score)),
        None => logger::info("Black team score: None"),
    }

    let team = String::from("White");
    let score = 255;
    scores.entry(team).or_insert(score);

    let white_team_score = scores.get(&String::from("White"));
    match white_team_score {
        Some(score) => logger::info(&format!("White team score: {}", score)),
        None => logger::info("White team score: None"),
    }

    let text = "hello world wonderful world";
    let mut map: HashMap<String, u32> = HashMap::new();

    for (i, word) in text.split_whitespace().enumerate() {
        let count = map.entry(word.to_string()).or_insert(0); // or_insertはキーに対する値の可変参照を返す
        logger::info(&format!("[{}]{}: {}", count, i, word));
        *count += 1; // 参照外し`*`をして値を更新することで、再度insertしなくても良くなる
    }
    logger::info(&format!("map: {:?}", map));
}
