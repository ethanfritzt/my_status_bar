use crate::status;

const BASE_PATH: &str = "/sys/class/power_supply";

// uevent events
//const POWER_SUPPLY_NAME: &str = "POWER_SUPPLY_NAME";
//const POWER_SUPPLY_TYPE: &str = "POWER_SUPPLY_TYPE";
const POWER_SUPPLY_STATUS: &str = "POWER_SUPPLY_STATUS";
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

// Font icons
const HALF: char = '\u{f12a2}';
const FULL: char = '\u{f12a3}';
const QUARTER: char = '\u{f12a1}';
const HALF_CHG: char = '\u{f12a5}';
const FULL_CHG: char = '\u{f12a6}';
const QUARTER_CHG: char = '\u{f12a4}';
const POW: char = '\u{f1c3b}';

#[derive(Default)]
struct BatteryInfo {
    current_charge: String,
    total_charge: String,
    status: String
}

trait GetBatteryInfo {
    fn find_battery(&self) -> std::path::PathBuf;
    fn get_battery_level(&self) -> String;
    fn get_battery_info(&mut self);
}

impl GetBatteryInfo for BatteryInfo {
    
    // returns the path for the in use battery
    fn find_battery(&self) -> std::path::PathBuf {
        let dir = std::fs::read_dir(BASE_PATH).unwrap();
        for entry in dir {
            let entry = entry.unwrap();
            let entry_type = entry.path().join("type");

            if let Ok(contents) = std::fs::read_to_string(&entry_type) {
                if contents.trim() == "Battery" {
                    return entry.path().join("uevent");    
                }
            }
        }
        return std::path::PathBuf::from(BASE_PATH);
    }

    fn get_battery_level(&self) -> String {
        let current_charge = &self.current_charge;
        let total_charge = &self.total_charge;

        let parsed_current_charge = current_charge.parse::<f64>().unwrap();
        let parsed_total_charge = total_charge.parse::<f64>().unwrap();

        let display_value = parsed_current_charge / parsed_total_charge * 100.0;

        display_value.to_string()
    }

    fn get_battery_info(&mut self) {
        let event_path = self.find_battery();
    
        // base path returned
        // set values and return
        if event_path.is_dir() {
            self.status = "POW".to_string();
            self.current_charge = 0.to_string();
            self.total_charge = 0.to_string();
            return;
        }

        let content = std::fs::read_to_string(event_path);
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

            if key == POWER_SUPPLY_STATUS.to_string() {
                if value == "Charging" {
                    self.status = "CHR".to_string()
                }
            }
        }
    }
}

pub fn get_batt_status_line() -> Vec<status::block::StatusLineBlock> {
    let mut battery_info = BatteryInfo::default();

    // set the values
    let _ = battery_info.get_battery_info();

    // return values
    let value = battery_info.get_battery_level();
    let status = battery_info.status;
    let battery_level = value.clone().parse::<f32>().unwrap();
    let icon: char = get_battery_icon(battery_level, status);
    let battery = status::block::StatusLineBlock {
        full_text: format_battery_level(battery_level),
        short_text: format_battery_level(battery_level),
        color: "#ccccccff".to_string(),
        background: "#111111ff".to_string(),
        border: "#222222ff".to_string(),
        border_top: 1,
        border_bottom: 1,
        border_left: 1,
        border_right: 1,
        min_width: 75,
        align: "center".to_string(),
        name: "bat".to_string(),
        instance: "bat".to_string(),
        urgent: false,
        separator: true,
        separator_block_width: 5,
        markup: "pango".to_string()
    };
    let battery_icon = status::block::StatusLineBlock {
        full_text: format!("<span size=\"{}\">{}</span>", 16 * 1024, icon),
        short_text: format!("{}", icon),
        color: get_battery_color(battery_level),
        background: "#111111ff".to_string(),
        border: "#222222ff".to_string(),
        border_top: 1,
        border_bottom: 1,
        border_left: 1,
        border_right: 1,
        min_width: 35,
        align: "center".to_string(),
        name: "bat".to_string(),
        instance: "bat".to_string(),
        urgent: false,
        separator: false,
        separator_block_width: 0,
        markup: "pango".to_string()
    };
    return vec![battery_icon, battery];
}

fn get_battery_icon(battery_level: f32, status: String) -> char {
    if status == "POW" {
        return POW;
    } else if status == "CHR" {
        if battery_level > 50.00 {
            return FULL_CHG;
        } else if battery_level > 25.00 {
            return HALF_CHG;
        } else {
            return QUARTER_CHG;
        }
    } else {
        if battery_level > 50.00 {
            return FULL;
        } else if battery_level > 25.00 {
            return HALF;
        } else {
            return QUARTER;
        }
    }
}

fn get_battery_color(battery_level: f32) -> String {
    if battery_level > 50.00 {
        return "#008000".to_string();
    } else if battery_level > 25.00 {
        return "#ffce1b".to_string();
    } else {
        return "#cd1c18".to_string();
    }
}

fn format_battery_level(value: f32) -> String {
    return format!("{:.2}%", value)
}
