fn main() {
	// Given values
	let initial_value: f64 = 510_000.0; // P
	let depreciation_rate: f64 = 5.0; // R
	let years: u32 = 3; // n

	// Calculate the depreciated value
	let final_value = initial_value * (1.0 + depreciation_rate / 100.0).powi(years as i32);

	// Output the result
	println!("The value of the TV after {} years is: ₦{:.2}", years, final_value);
}