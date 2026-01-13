use crate::{Node, Valuation};

pub fn try_print_truth_table(formula: &str) -> Result<(), &'static str> {
    let ast = Node::try_from(formula)?;
    let mut valuation = Valuation::try_from(formula)?;

    valuation.print_table_header();
    for i in 0..(1 << valuation.len()) {
        valuation.update_int(i);
        valuation.print_table_body();

        println!("| {} |", ast.eval(&valuation)? as u8);
    }
    Ok(())
}

#[allow(dead_code)]
pub fn print_truth_table(formula: &str) {
    if let Err(msg) = try_print_truth_table(formula) {
        println!("{msg}");
    }
}
