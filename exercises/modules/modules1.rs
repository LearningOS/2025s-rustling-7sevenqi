// modules1.rs

mod sausage_factory {
    // 私有函数，外部不可见
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 添加 pub 关键字使函数对外可见
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
