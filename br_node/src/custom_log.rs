use log::kv::Key;
use std::io::Write;

pub static NID: &str = "NID";

pub fn init() {
    env_logger::builder()
        .format(|buf, record| {
            let ts = buf.timestamp_millis();
            let env = match record.key_values().get(Key::from(NID)) {
                Some(val) => format!(" {}-{}", NID, val),
                _ => String::new(),
            };
            let level = record.level();
            let warn_style = buf.default_level_style(level);

            writeln!(
                buf,
                "[{} {warn_style}{:5}{warn_style:#} {}{}] {}",
                ts,
                level,
                record.module_path().unwrap_or("").trim(),
                env,
                record.args()
            )
        })
        .init();
}
