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
