use std::collections::HashMap;

enum Score {
    INT(u32),
    FLOAT{value : f32},
}

fn main() {
    insert();
    collect();
    accessing();
    looping();
    replacing();
    insert_new();
    increase_score();
}

fn insert() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), Score::INT(10));
    scores.insert(String::from("Blue"), Score::FLOAT{value : 11.2f32});


    match scores.get("Blue") {
        Some(Score::INT(value)) => println!("Blue score: {}", value),
        Some(Score::FLOAT{value}) => println!("Blue score: {}", value),
        None => println!("Not found")
    }
}

fn collect() {
    let teams = vec![String::from("Red"), String::from("Blue")];
    let initial_scores = vec![Score::INT(10), Score::FLOAT{value: 20.2}];

    // Clones the content into an Iterable structure, withtou the usage of refference.
    //let mut iter = teams.into_iter();
    //assert_eq!(Some(1), iter.next());
    let scores: HashMap<_,_> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // The iter method returns a slice of refference values
    //let mut iterator = teams.iter()
    //assert_eq!(iterator.next(), Some(&1));

    match scores.get("Blue") {
        Some(Score::INT(value)) => println!("Blue score: {}", value),
        Some(Score::FLOAT{value}) => println!("Blue score: {}", value),
        None => println!("Not found")
    }
}

fn accessing() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 21);

    let blue = String::from("Blue");
    let score = scores.get(&blue);
    println!("Blue score: {:?}", score);
}

fn looping() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Blue"), 31);

    for (key, value) in scores {
        println!("{} score: {}", key, value);
    }
}

fn replacing() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 20);
    scores.insert(String::from("Red"), 25);

    println!("Red score: {:?}", scores.get("Red"));
}

fn insert_new() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Red"), 20);

    scores.entry(String::from("Yellow")).or_insert(10);
    scores.entry(String::from("Blue")).or_insert(30);
    scores.entry(String::from("Red")).or_insert(10);

    println!("{:?}", scores);
}

fn increase_score() {
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}