// Define a Circle struct with a radius
struct Circle {
    radius: f64, // Radius of the circle as a 64-bit floating-point number
}

// Implementation block for Circle
impl Circle {
    // Constructor function to create a new Circle
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Function to calculate the area of the circle
    fn area(&self) -> f64 {
        let pi = 3.14;
        pi * self.radius * self.radius
    }

    // Function to calculate the circumference of the circle
    fn circumference(&self) -> f64 {
        let pi = 3.14;
        2.0 * pi * self.radius
    }
}

fn main() {
    // Create a new circle with radius 5.0
    let circle = Circle::new(5.0);

    // Print the area of the circle
    println!("Circle area: {}", circle.area());

    // Print the circumference of the circle
    println!("Circle circumference: {}", circle.circumference());

    // Test cases to verify the methods work as expected
    assert!((Circle::new(5.0).area() - 78.5).abs() < 0.01);
    assert!((Circle::new(5.0).circumference() - 31.4).abs() < 0.01);

}




/* 
Task 2:
//struct: allows to package together related values of different data types.
struct Rectangle { //define a new customm data type (rectangle) using struct
    width: f64, // Width of the rectangle as a 64-bit floating-point number
    height: f64, // Height of the rectangle as a 64-bit floating-point number
}
//to define methods within the context of a Rectangle struct, we use implementation block (impl)
impl Rectangle {
     // Constructor method: create a new rectangle with given width and height
    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
    // Calculate the area of the rectangle
    fn area(&self) -> f64 {
        self.width * self.height
    }
    // Calculate the perimeter of the rectangle
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
    // Check if the rectangle is actually a square
    fn is_square(&self) -> bool {
        self.width == self.height

    }
}
fn main() {
    let rect = Rectangle::new(10.0, 5.0); //declare a variable name rect and intstantiate a Rectangle and give it values for each field i.e width & height.
    println!("Area: {}", rect.area());    // Print the area
    println!("Perimeter: {}", rect.perimeter()); // Print the perimeter
    println!("Is square? {}", rect.is_square()); // Check if it is a square

    // Test cases to verify the methods work as expected
    assert!(Rectangle::new(5.0, 5.0).is_square());
    assert!(!Rectangle::new(5.0, 6.0).is_square());
}
*/
