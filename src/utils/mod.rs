pub fn init_logger() {
    use chrono::Local;
    use std::io::Write;

    let env = env_logger::Env::default()
        .filter_or(env_logger::DEFAULT_FILTER_ENV, "info");
    env_logger::Builder::from_env(env)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{}] {} [{}] {}",
                record.level(),
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                "Web",
                &record.args()
            )
        })
        .init();
}