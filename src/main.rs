use std::env;
use std::fs;

fn main() {
    let word: String = String::from("hello world");
    let index = second_word(&word);
    println!("index is {}", index);
    get_content();
    test_mut();
}

fn get_content (){
    let args = env::args().collect::<Vec<String>>();
    let query = args[1].clone();
    let filename = args[2].clone();
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    println!("With text:\n{}: {}", query, contents);
}

fn second_word(s: &String)-> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}

fn test_mut(){
    let a = String::from("hello");
    let r = &a[0..2];
    println!("r: {} : {}", a, r);

}