// box1.rs


#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // 用Box包裹递归类型[6,7](@ref)
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil // 空列表直接返回终止节点[7](@ref)
}

pub fn create_non_empty_list() -> List {
    List::Cons(
        1,
        Box::new(List::Cons(
            2,
            Box::new(List::Cons(3, Box::new(List::Nil)))
        ))
    )
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
