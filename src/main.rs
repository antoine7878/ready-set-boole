use ready::print_truth_table;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let formula = "AB&C|";
    println!("\nTruth table for: '{formula}'");
    print_truth_table(formula);

    let formula = "ABC&&";
    println!("\nTruth table for: '{formula}'");
    print_truth_table(formula);

    let formula = "ABC||";
    println!("\nTruth table for: '{formula}'");
    print_truth_table(formula);

    Ok(())
}
