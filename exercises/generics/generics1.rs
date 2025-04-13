// generics1.rs


fn main() {
    let mut shopping_list: Vec<&str> = Vec::new(); // 修复此处
    shopping_list.push("milk");
}
