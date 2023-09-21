#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect = Rectangle {
        width: dbg!(412 * scale),
        height: 419,
    };
    println!("{:?}", rect);
    println!("{}", calculate_rectangle_area(&rect));
}

fn calculate_rectangle_area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
