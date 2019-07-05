fn main() {
    let mut s = String::from("Hello, world!");
    println!("Hello, world!");
    println!("{}", s);
    println!("{}", s);
    string_to_word(&s[0..], 0);
}

fn word_tp_pig_latin(word: &str) -> &str{
    
    word
}

fn string_to_word(sentence: &str, begin: u32) -> (&str, u32, u32){
    let dividers = (
        ' ', '!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-',
        '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_',
        '`', '{', '|', '}', '~');
    for letter in sentence.chars() {
        println!("{}", letter);
    }
    ("122", 12, 34)
}