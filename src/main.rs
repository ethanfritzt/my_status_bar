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
    let json_body: Vec<status::block::StatusLineBlock> = vec![
        utils::battery::get_batt_status_line(),
        utils::time::get_time_status_line()
    ].into_iter().flatten().collect();

    print!("{},", serde_json::to_string(&json_body).unwrap());
    let _ = std::io::stdout().flush();
}
