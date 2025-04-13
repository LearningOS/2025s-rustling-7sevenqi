// strings2.rs


fn main() {
    let word = String::from("green"); // 保持这行不变
    if is_a_color_word(&word) {      // 添加引用符号 &
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
