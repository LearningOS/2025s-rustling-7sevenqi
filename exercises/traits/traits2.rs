// traits2.rs

trait AppendBar {
    fn append_bar(self) -> Self;
}

// 为字符串向量实现追加功能
impl AppendBar for Vec<String> {
    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar")); // 关键点：获取所有权后修改
        self
    }
}
// TODO: Implement trait `AppendBar` for a vector of strings.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), String::from("Bar"));
        assert_eq!(foo.pop().unwrap(), String::from("Foo"));
    }
}
