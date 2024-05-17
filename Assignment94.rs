fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string_slice("rust is fun!");
    string_slice("nice weather");
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string_slice("mY sHiFt KeY iS sTiCkY".to_lowercase().as_str());
}
