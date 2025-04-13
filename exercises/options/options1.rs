// options1.rs

    fn maybe_icecream(time_of_day: u16) -> Option<u16> {
        match time_of_day {
            0..=21 => Some(5),   // 0-21点返回5块
            22 | 23 => Some(0),  // 22-23点返回0块
            _ => None            // 其他时间返回无效
        }
    }
    
    #[cfg(test)]
mod tests {
    use super::maybe_icecream;  // 使用 super 访问父模块
    
    #[test]
    fn raw_value() {
        let icecreams = maybe_icecream(12);
        assert_eq!(icecreams.unwrap(), 5);
    }
}
