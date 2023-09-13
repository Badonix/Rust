fn main() {
    let celsius = farenheit_to_celsius(141.32);
    println!("{celsius}");
    let nth = nth_fibonacci(6);
    println!("{}", nth);
}
fn farenheit_to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) * (5.0 / 9.0)
}
fn nth_fibonacci(n: u32) -> u32 {
    if n == 1 {
        1
    } else if n == 2 {
        1
    } else {
        let mut prev = 1;
        let mut current = 1;
        for _ in 2..n {
            let new_current = prev + current;
            prev = current;
            current = new_current;
        }
        current
    }
}
