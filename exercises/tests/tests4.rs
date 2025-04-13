// tests4.rs


struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {
    // Only change the test functions themselves
    pub fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            panic!("Rectangle width and height cannot be negative!")
        }
        Rectangle {width, height}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10);  // 检查宽度字段[6](@ref)
        assert_eq!(rect.height, 20); // 检查高度字段[6](@ref)
    }

    #[test]
    #[should_panic(expected = "cannot be negative")] // 验证特定panic信息[3,5](@ref)
    fn negative_width() {
        Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic(expected = "cannot be negative")]
    fn negative_height() {
        Rectangle::new(10, -10);
    }
}
