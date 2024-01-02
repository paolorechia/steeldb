use env_logger::Env;

pub fn logger_init() {
    let env = Env::default()
        .filter_or("STEELDB_LOG_LEVEL", "info")
        .write_style_or("STEELDB_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}
