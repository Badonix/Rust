fn main() {
    let celsius = farenheit_to_celsius(141.32);
    println!("{celsius}");
}
fn farenheit_to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * (5.0 / 9.0)
}
