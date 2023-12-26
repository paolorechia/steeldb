use steeldb::repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.main_loop();
}
