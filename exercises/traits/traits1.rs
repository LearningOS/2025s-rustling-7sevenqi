// traits1.rs

trait AppendBar {
    fn append_bar(self) -> Self;
}

// 关键实现：通过 push_str 修改字符串
impl AppendBar for String {
    fn append_bar(mut self) -> Self { // 获取 mut 所有权
        self.push_str("Bar");        // 追加内容
        self                         // 返回修改后的 String
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
