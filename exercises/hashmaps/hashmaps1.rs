// hashmaps1.rs

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new(); // 初始化哈希映射

    // 插入香蕉（已给出）
    basket.insert(String::from("banana"), 2);
    
    // 添加新水果（至少两种）
    basket.insert(String::from("apple"), 3);  // 添加苹果
    basket.insert(String::from("mango"), 1);  // 添加芒果
    
    basket
}
