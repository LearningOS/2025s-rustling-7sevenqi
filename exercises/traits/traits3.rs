// traits3.rs

pub trait Licensed {
    // 关键点：添加默认实现
    fn licensing_info(&self) -> String {
        String::from("Some information") // 统一返回的字符串
    }
}

struct SomeSoftware {
    version_number: i32,
}

struct OtherSoftware {
    version_number: String,
}

// 使用默认实现
impl Licensed for SomeSoftware {} 
impl Licensed for OtherSoftware {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_licensing_info_the_same() {
        let licensing_info = String::from("Some information");
        let some_software = SomeSoftware { version_number: 1 };
        let other_software = OtherSoftware {
            version_number: "v2.0.0".to_string(),
        };
        assert_eq!(some_software.licensing_info(), licensing_info);
        assert_eq!(other_software.licensing_info(), licensing_info);
    }
}
