fn main() {
    lalrpop_dir("src/calc");
    lalrpop_dir("src/tiger");
}

fn lalrpop_dir(dir: &str) {
    lalrpop::Configuration::new()
        .set_in_dir(dir)
        .process()
        .unwrap();
}
