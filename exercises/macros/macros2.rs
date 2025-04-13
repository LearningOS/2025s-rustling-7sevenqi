// macros2.rs


macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!(); // ✅ 此时宏已定义
}
