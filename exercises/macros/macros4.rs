// macros4.rs

#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // 添加分号分隔分支
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    }; // 添加分号分隔分支
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
