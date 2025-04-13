// macros3.rs


mod macros {
    #[macro_export] // 关键导出属性
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!(); // 直接调用无需模块前缀
}
