fn main() {
    let amounts = (450000.0,1500000.0,750000.0,2850000.0,250000.0);
    
    let total: f64 = amounts.iter().sum();
    let average = total / amounts.len() as f64;
    
    println!("Total Amount: {}", total);
    println!("Total Average: {}", average);
}