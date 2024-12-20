fn main() {
    // Given values
    let principal: f64 = 520_000_000.0; // Principal amount in Naira
    let rate: f64 = 10.0; // Annual interest rate in percentage
    let years: i32 = 5; // Time period in years

    let amount = principal * (1.0 + rate / 100.0).powi(years);

    // Calculate the compound interest (CI)
    let compound_interest = amount - principal;

    // Display the results
    println!("Principal (P): ₦{:.2}", principal);
    println!("Rate (R): {:.2}%", rate);
    println!("Time (n): {} years", years);
    println!("Total Amount (A): ₦{:.2}", amount);
    println!("Compound Interest (CI): ₦{:.2}", compound_interest);
}
