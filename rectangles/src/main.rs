struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 412,
        height: 419,
    };

    println!("{}", calculate_rectangle_area(&rect));
}

fn calculate_rectangle_area(dimensions: &Rectangle) -> u32 {
    dimensions.height * dimensions.width
}
