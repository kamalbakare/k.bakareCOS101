// Rust Program to Find Roots of a Quadratic Equation

use std::io;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();

    println!("Enter the value for a");
    io::stdin().read_line(&mut a).expect("Failed to read input");
    let a:f32 = a.trim().parse().expect("Failed to input");

    println!("Enter the value for b");
    io::stdin().read_line(&mut b).expect("Failed to read input");
    let b:f32 = b.trim().parse().expect("Failed to input");

    println!("Enter the value for c");
    io::stdin().read_line(&mut c).expect("Failed to read input");
    let c:f32 = c.trim().parse().expect("Failed to input");

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {

        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: {:.2} and {:.2}", root1, root2);
    } else if discriminant == 0.0 {

        let root = -b / (2.0 * a);
        println!("One real root: {:.2}", root);
    } else {

        println!("No real roots (discriminant is negative).");
    }
}