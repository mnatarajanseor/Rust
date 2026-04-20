#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

struct Cicle {
    radius: f64,
}
impl Cicle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}   
fn main() {
    println!("Hello, world!");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
   
    println!("The area of the rectangle is {} square pixels.", area(&rect1));
    let cicle1 = Cicle {
        radius: 5.0,
    };
    println!("The area of the cicle is {} square pixels.", cicle1.area());
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}   

