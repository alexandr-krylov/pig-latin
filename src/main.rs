fn main() {
    let mut s = String::from("Hello, world!");
    println!("Hello, world!");
    println!("{}", s);
}

fn word_tp_pig_latin(word: &str) -> &str{
    let dividers = (
        ' ', '!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-',
        '.', '/', ':', ';', '<', '=', '>', '?', '@', '[', '\\', ']', '^', '_',
        '`', '{', '|', '}', '~');
    word
}