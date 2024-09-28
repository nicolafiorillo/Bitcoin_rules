use log::kv::Key;
use std::io::Write;

pub static NID: &str = "NID";

pub fn init() {
    env_logger::builder()
        .format(|buf, record| {
            let ts = buf.timestamp_millis();
            let env = record
                .key_values()
                .get(Key::from(NID))
                .map_or(String::new(), |val| format!(" {}-{}", NID, val));
            let level = record.level();
            let level_style = buf.default_level_style(level);

            writeln!(
                buf,
                "[{} {level_style}{:5}{level_style:#} {}{}] {}",
                ts,
                level,
                record.module_path().unwrap_or("").trim(),
                env,
                record.args()
            )
        })
        .init();
}
