// structs3.rs

#[derive(Debug)]
struct Package {
    sender_country: String,
    recipient_country: String,
    weight_in_grams: i32,
}

impl Package {
    fn new(sender_country: String, recipient_country: String, weight_in_grams: i32) -> Package {
        if weight_in_grams <= 0 {
            panic!("Can not ship a weightless package.")
        } else {
            Package {
                sender_country,
                recipient_country,
                weight_in_grams,
            }
        }
    }

    // 判断是否为国际包裹
    fn is_international(&self) -> bool {
        self.sender_country != self.recipient_country
    }

    // 计算运输费用
    fn get_fees(&self, cents_per_gram: i32) -> i32 {
        self.weight_in_grams * cents_per_gram
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn fail_creating_weightless_package() {
        let sender_country = String::from("Spain");
        let recipient_country = String::from("Austria");
        Package::new(sender_country, recipient_country, -2210);
    }

    #[test]
    fn create_international_package() {
        let package = Package::new(
            String::from("Spain"),
            String::from("Russia"),
            1200
        );
        assert!(package.is_international());
    }

    #[test]
    fn create_local_package() {
        let country = String::from("Canada");
        let package = Package::new(
            country.clone(),
            country,
            1200
        );
        assert!(!package.is_international());
    }

    #[test]
    fn calculate_transport_fees() {
        let package = Package::new(
            String::from("Spain"),
            String::from("Spain"),
            1500
        );
        assert_eq!(package.get_fees(3), 4500);
        assert_eq!(package.get_fees(6), 9000);
    }
}
