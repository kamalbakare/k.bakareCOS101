fn main() {
	// Sales amounts
	let amounts = [450_000.0, 1_500_000.0, 750_000.0, 2_850_000.0, 250_000.0];

	// Calculate total and average
	let total: f64 = amounts.iter().sum();
	let average = total / amounts.len() as f64;

	//Output results
	println!("Total Sales Amount: {:.2}", total);
	println!("Average Sales Amount: {:.2}", average);
}