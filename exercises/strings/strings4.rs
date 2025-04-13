// strings4.rs

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");                    // [2,7](@ref)
    string("red".to_string());                // [3,7](@ref)
    string(String::from("hi"));               // [2,7](@ref)
    string("rust is fun!".to_owned());        // [7,8](@ref)
    string("nice weather".into());            // [3,7](@ref)
    string(format!("Interpolation {}", "Station")); // [3,7](@ref)
    string_slice(&String::from("abc")[0..1]); // [7,8](@ref)
    string_slice("  hello there ".trim());    // [3,7](@ref)
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // [3,7](@ref)
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // [3,7](@ref)
}
