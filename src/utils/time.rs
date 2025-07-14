use crate::status;

const UTCMINUS4OFFSET: u64 = 14400;
const HOURSINDAY: u64 = 24;
const SECONDSINHOUR: u64 = 3600;
const SECONDSINMINUTE: u64 = 60;

fn get_time() -> String {
   let now = std::time::SystemTime::now();
   let since_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();
   let as_seconds = since_epoch.as_secs();
   
   return format_time(as_seconds);
}

fn format_time(seconds: u64) -> String {
    let hours = ((seconds - UTCMINUS4OFFSET) / SECONDSINHOUR) % HOURSINDAY;
    let minutes = (seconds / SECONDSINMINUTE) % SECONDSINMINUTE;
    return format!("{:02}:{:02}", hours, minutes);
}

pub fn get_time_status_line() -> status::block::StatusLineBlock {
    let current_time  = status::block::StatusLineBlock {
        full_text: get_time(),
        short_text: get_time(),
        color: "#ccccccff".to_string(),
        background: "#111111ff".to_string(),
        border: "#222222ff".to_string(),
        border_top: 1,
        border_bottom: 1,
        border_left: 1,
        border_right: 1,
        min_width: 100,
        align: "center".to_string(),
        name: "clock".to_string(),
        instance: "edt".to_string(),
        urgent: false,
        separator: true,
        separator_block_width: 5,
        markup: "none".to_string()
    };

    return current_time
}
