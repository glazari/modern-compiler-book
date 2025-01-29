fn main() {
    lalrpop_dir("src/calc");
}

fn lalrpop_dir(dir: &str) {
    lalrpop::Configuration::new()
        .set_in_dir(dir)
        .process()
        .unwrap();
}
