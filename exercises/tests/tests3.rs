// tests3.rs


pub fn is_even(num: i32) -> bool {
    num % 2 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true_when_even() {
        // 测试偶数场景（例如4）
        assert!(is_even(4)); // 断言返回true[6,7](@ref)
    }

    #[test]
    fn is_false_when_odd() {
        // 测试奇数场景（例如5）
        assert!(!is_even(5)); // 断言返回false[6,8](@ref)
    }

    #[test]
    fn test_odd_case() {
        // 题目要求的额外测试
        assert_eq!(is_even(5), false); // 使用assert_eq!明确期望值[5,7](@ref)
    }
}
