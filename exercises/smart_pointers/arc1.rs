// arc1.rs

#![forbid(unused_imports)]
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(numbers);  // 包装原始数据到Arc
    
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);  // 克隆Arc指针
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers
                .iter()
                .filter(|&&n| n % 8 == offset)
                .sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
