use crate::status;

fn get_time() -> String {
   let now = chrono::Local::now();
   now.format("%H:%M").to_string()
}

pub fn get_time_status_line() -> Vec<status::block::StatusLineBlock> {
    let current_time  = status::block::StatusLineBlock {
        full_text: get_time(),
        min_width: 100,
        name: "clock".to_string(),
        instance: "edt".to_string(),
        separator: true,
        separator_block_width: 5,
        ..Default::default()
    };

    vec![current_time]
}
