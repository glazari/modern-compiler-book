use tigerrs::calc;
use tigerrs::straight_line;

fn main() {
    println!("Running straight-line interpreter");
    straight_line::main();
    println!("Running calc interpreter");
    calc::main();
}
