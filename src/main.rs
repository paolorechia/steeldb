use steeldb::SteelDB;
use steeldb_core::Repl;

fn main() {
    let database = SteelDB::new();
    let mut repl = Repl::new(Box::new(database));
    repl.main_loop();
}
