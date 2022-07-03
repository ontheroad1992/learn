use std::collections::HashMap;

pub fn test() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("scores: {:?}", scores);

    // 空值的插入
    scores.entry(String::from("Red")).or_insert(233);
    println!("scores: {:?}", scores);

    // let team_name = String::from("Blue");
    // let score = scores.get(&String::from("Blue"));
    println!("Blue: {:?}", scores.get(&String::from("Blue")));
    for (key, value) in &scores {
        println!("{}:{}", key, value)
    }

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let inital_scores = vec![10, 50];
    let scores1: HashMap<_, _> = teams.into_iter().zip(inital_scores.into_iter()).collect();
    println!("scores: {:?}", scores1);

    test_owner();

    calc_word();
}

fn test_owner() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, field_value);
    println!("map: {:?}", map);
    println!("field_name: {}", field_name); // 如果 field_name不是使用借用的方式， 所有权会被被拿走
}

fn calc_word() {
    let text = "hello world herman world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    print!("map: {:?}", map)
}
