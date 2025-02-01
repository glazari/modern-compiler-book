use tigerrs::calc;
use tigerrs::straight_line;
use tigerrs::tiger;



fn main() {
    println!("{}", "*".repeat(80));
    println!("Running straight-line interpreter");
    straight_line::main();
    println!("{}", "*".repeat(80));
    println!("Running calc interpreter");
    calc::main();
    println!("{}", "*".repeat(80));
    println!("Running tiger interpreter");
    tiger::main();
}
