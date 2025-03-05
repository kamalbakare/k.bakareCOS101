use std::io;

fn main() {
    println!("Choose a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice:u32 = choice.trim().parse().expect("Invalid input");

    match choice {
        1 => trap_area(),
        2 => rhomb_area(),
        3 => par_area(),
        4 => cube_area(),
        5 => cyl_vol(),
        _ => println!("Invalid choice"),
    }
}

fn trap_area() {
    let mut h = String::new();
    println!("Enter height");
    io::stdin().read_line(&mut h).expect("Not a valid input");
    let h:f64 = h.trim().parse().expect("Not an integer");

    let mut b1 = String::new();
    println!("Enter base 1");
    io::stdin().read_line(&mut b1).expect("Not a valid input");
    let b1:f64 = b1.trim().parse().expect("Not an integer");

    let mut b2 = String::new();
    println!(" Enter base 2");
    io::stdin().read_line(&mut b2).expect("Not a valid input");
    let b2:f64 = b2.trim().parse().expect("Not an integer");

    let area:f64 = h/2.0 * (b1 + b2);
    println!("The Area of Trapezium is: {}", area);
}
fn rhomb_area() {
    let mut d1 = String::new();
    println!("Enter diagonal 1");
    io::stdin().read_line(&mut d1).expect("Not a valid input");
    let d1:f64 = d1.trim().parse().expect("Not an integer");

    let mut d2 = String::new();
    println!("Enter diagonal 2");
    io::stdin().read_line(&mut d2).expect("Not a valid input");
    let d2:f64 = d2.trim().parse().expect("Not an integer");

    let area:f64 = 0.5 * d1 * d2;
    println!("The Area of Rhombus is: {}", area);
}

fn par_area() {
    let mut base = String::new();
    println!("Enter base");
    io::stdin().read_line(&mut base).expect("Not a valid input");
    let base:f64 = base.trim().parse().expect("Not an integer");

    let mut altitude = String::new();
    println!("Enter altitude");
    io::stdin().read_line(&mut altitude).expect("Not a valid input");
    let altitude:f64 = altitude.trim().parse().expect("Not an integer");

    let area:f64 = altitude * base;
    println!("The Area of Parallelogram is: {}", area);
}

fn cube_area() {
    let mut length = String::new();
    println!("Enter length");
    io::stdin().read_line(&mut length).expect("Not a valid input");
    let length:f64 = length.trim().parse().expect("Not an integer");

    let area:f64 = 6.0 * length * length;
    println!("The Area of Cube is: {}", area);
}

fn cyl_vol() {
    let mut height = String::new();
    println!("Enter height");
    io::stdin().read_line(&mut height).expect("Not a valid input");
    let height:f64 = height.trim().parse().expect("Not an integer");

    let mut r = String::new();
    println!("Enter radius");
    io::stdin().read_line(&mut r).expect("Not a valid input");
    let r:f64 = r.trim().parse().expect("Not an integer");

    let pi:f64 = 22.0/7.0;
    let volume:f64 = pi * r * r *height;
    println!("The Volume of Cylinder is: {}", volume);
}