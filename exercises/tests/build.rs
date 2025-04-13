
fn main() {
    let package_name = std::env::var("CARGO_PKG_NAME").unwrap();

    if package_name == "tests7" {
        // In tests7, set the TEST_FOO environment variable with the current timestamp
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        println!("cargo:rustc-env=TEST_FOO={}", timestamp);
    } else if package_name == "tests8" {
        // In tests8, enable the "pass" feature
        println!("cargo:rustc-cfg=feature=\"pass\"");
    }
}
