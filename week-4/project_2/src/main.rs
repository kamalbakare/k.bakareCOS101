// Rust program to determine the annual incentive for an employee based on their experience and age

use std::io;

fn main() {
    let mut age_inp = String::new();
    let mut exp_inp = String::new();


    println!("Enter the age of the employee");
    io::stdin().read_line(&mut age_inp).expect("Not an integer");
    let age:i32 = age_inp.trim().parse().expect("Not a valid age");

    println!("Is the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut exp_inp).expect("Not an option");
    let binding = exp_inp.to_lowercase();
    let exp = binding.trim();
    if age >=40 && exp == "yes" {
        let incentive = 1_560_000;
        println!("Your annual incentive is N{}", incentive);
    } else if age >= 30 && age < 40 && exp =="yes"{
        let incentive = 1_480_000;
        println!("Your annual incentive is N{}", incentive);

    } else if age < 28 && exp == "yes" {
        let incentive = 1_300_000;
        println!("Your annual incentive is N{}", incentive);
    } else if exp != "yes" && exp != "no" {
        println!("Please try again and enter yes or no");
    }
    else{
        let incentive = 100_000;
        println!("Your annual incentive is N{}", incentive);
    }
}