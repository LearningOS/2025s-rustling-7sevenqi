// tests2.rs


#[cfg(test)]
mod tests {
    #[test]
    fn you_can_assert_eq() {
        // 通过测试的版本（参数相等）
        assert_eq!(42, 42);
        
        // 使测试失败的版本（参数不相等）
        // assert_eq!(42, 0);
    }
}
