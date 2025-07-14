use crate::status;

const PATH: &str = "/sys/class/power_supply/BAT0/uevent";
//const POWER_SUPPLY_NAME: &str = "POWER_SUPPLY_NAME";
//const POWER_SUPPLY_TYPE: &str = "POWER_SUPPLY_TYPE";
//const POWER_SUPPLY_STATUS: &str = "POWER_SUPPLY_STATUS";
//const POWER_SUPPLY_PRESENT: &str = "POWER_SUPPLY_PRESENT";
//const POWER_SUPPLY_TECHNOLOGY: &str = "POWER_SUPPLY_TECHNOLOGY";
//const POWER_SUPPLY_CYCLE_COUNT: &str = "POWER_SUPPLY_CYCLE_COUNT";
//const POWER_SUPPLY_VOLTAGE_MIN_DESIGN: &str = "POWER_SUPPLY_VOLTAGE_MIN_DESIGN";
//const POWER_SUPPLY_VOLTAGE_NOW: &str = "POWER_SUPPLY_VOLTAGE_NOW";
//const POWER_SUPPLY_POWER_NOW: &str = "POWER_SUPPLY_POWER_NOW";
const POWER_SUPPLY_ENERGY_FULL_DESIGN: &str = "POWER_SUPPLY_ENERGY_FULL_DESIGN";
//const POWER_SUPPLY_ENERGY_FULL: &str = "POWER_SUPPLY_ENERGY_FULL";
const POWER_SUPPLY_ENERGY_NOW: &str = "POWER_SUPPLY_ENERGY_NOW";
//const POWER_SUPPLY_CAPACITY: &str = "POWER_SUPPLY_CAPACITY";
//const POWER_SUPPLY_CAPACITY_LEVEL: &str = "POWER_SUPPLY_CAPACITY_LEVEL";
//const POWER_SUPPLY_MODEL_NAME: &str = "POWER_SUPPLY_MODEL_NAME";
//const POWER_SUPPLY_MANUFACTURER: &str = "POWER_SUPPLY_MANUFACTURER";
//const POWER_SUPPLY_SERIAL_NUMBER: &str = "POWER_SUPPLY_SERIAL_NUMBER";

#[derive(Default)]
struct BatteryInfo {
    current_charge: String,
    total_charge: String
}

trait GetBatteryInfo {
    fn get_battery_level(&self) -> String;
    fn get_battery_info(&mut self);
}

impl GetBatteryInfo for BatteryInfo {
    fn get_battery_level(&self) -> String {
        let current_charge = &self.current_charge;
        let total_charge = &self.total_charge;

        let parsed_current_charge = current_charge.parse::<f64>().unwrap();
        let parsed_total_charge = total_charge.parse::<f64>().unwrap();

        let display_value = parsed_current_charge / parsed_total_charge * 100.0;

        display_value.to_string()
    }

    fn get_battery_info(&mut self) {
        let content = std::fs::read_to_string(PATH);
        let lines = content.unwrap();

        for line in lines.lines() {
            let split_str: Vec<&str> = line.split("=").collect();
            let key = split_str[0];
            let value = split_str[1];

            if key == POWER_SUPPLY_ENERGY_NOW.to_string() {
                self.current_charge = value.to_string();
            }

            // full design gets us the full capacity of the battery
            // before degradation?
            // NOTE: this is what i3status uses
            if key == POWER_SUPPLY_ENERGY_FULL_DESIGN.to_string() {
                self.total_charge = value.to_string();
            }
        }
    }
}

pub fn get_batt_status_line() -> status::block::StatusLineBlock {
    let battery_level = status::block::StatusLineBlock {
        full_text: format_battery_level(),
        short_text: format_battery_level(),
        color: "#ccccccff".to_string(),
        background: "#111111ff".to_string(),
        border: "#222222ff".to_string(),
        border_top: 1,
        border_bottom: 1,
        border_left: 1,
        border_right: 1,
        min_width: 100,
        align: "center".to_string(),
        name: "bat".to_string(),
        instance: "bat".to_string(),
        urgent: false,
        separator: true,
        separator_block_width: 5,
        markup: "none".to_string()
    };
    return battery_level
}

fn format_battery_level() -> String {
    let mut battery_info = BatteryInfo::default();

    // set the values
    let _ = battery_info.get_battery_info();

    // return values
    let value = battery_info.get_battery_level();

    return format!("{:.2}%", value.parse::<f32>().unwrap())
}
