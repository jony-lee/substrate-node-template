// 定义一个 Shape trait
trait Shape {
    fn area(&self) -> f64;
}

// 实现 Shape trait for Circle
struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

// 实现 Shape trait for Triangle
struct Triangle {
    base: f64,
    height: f64,
}

impl Shape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

// 实现 Shape trait for Square
struct Square {
    side: f64,
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}

// 打印图形面积的函数
fn print_area<T: Shape>(shape: T) {
    println!("The area of the shape is: {}", shape.area());
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let triangle = Triangle { base: 4.0, height: 7.0 };
    let square = Square { side: 6.0 };

    print_area(circle);
    print_area(triangle);
    print_area(square);
}
