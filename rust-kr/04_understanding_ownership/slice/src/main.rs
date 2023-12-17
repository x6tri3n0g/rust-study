fn main() {
    // let s: String = String::from("hello world!");
    
    // let first_word = first_words(&s);
    // println!("{}", first_word);

    let my_string = String::from("hello world");
    // 'first_word'는 String의 일부 혹은 전체 슬라이스에 대해 작동함
    let word1 = first_word(&my_string[0..6]);
    println!("{}", word1);
    let word2 = first_word(&my_string[..]);
    println!("{}", word2);
    // 또한 'first_word'는 String의 전체 슬라이스와 동일한 String의 참조자에 대해서도 다룹니다.
    let word3 = first_word(&my_string);
    println!("{}", word3);

    let my_string_literal = "hello world";
    // 문자열 리터럴은 곧 문자열 슬라이스이므로, 아래 코드도 슬라이스 문법 없이 작동합니다.
    let word4 = first_word(my_string_literal);
    println!("{}", word4);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}