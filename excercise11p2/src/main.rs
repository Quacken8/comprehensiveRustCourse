fn main() {
    let v: Vec<String> = vec![String::from("foo"), String::from("bar")];

    for word in &v {
        let x:&String = word;
        println!("word: {word}");
    }

    for word in v {
        let x:String = word;
        println!("word: {x}");
    }
}