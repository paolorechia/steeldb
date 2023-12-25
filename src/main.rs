use steeldb::repl::Repl;
// use calculator::calculator::TermParser;

// lalrpop_mod!(pub select); // synthesized by LALRPOP

fn main() {
    let mut repl = Repl::new();
    repl.main_loop();
}
