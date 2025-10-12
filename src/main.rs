mod utils;
mod status;

use std::io::Write;

fn main() {
    // create the header
    output_header();

    // run process
    loop {
        run_process();

        std::thread::sleep(std::time::Duration::from_millis(3000));
    }
}

fn output_header() {
    // header
    println!("{{ \"version\": 1 }}");

    // start the infinite json array
    println!("[");
}

fn run_process() {
    let battery_level = utils::battery::get_batt_status_line();
    let current_time = utils::time::get_time_status_line();

    let json_body = serde_json::json!([battery_level[0], battery_level[1], current_time]);

    print!("{},", json_body);
    let _ = std::io::stdout().flush();
}
