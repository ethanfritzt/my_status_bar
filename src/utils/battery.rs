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
const POW: char = '\u{F06A5}';

#[derive(Default)]
struct BatteryInfo {
    battery_path: Option<std::path::PathBuf>,
    current_charge: String,
    total_charge: String,
    status: String
}

impl BatteryInfo {
    
    // returns the path for the in use battery
    fn find_battery() -> Option<std::path::PathBuf> {
        let dir = std::fs::read_dir(BASE_PATH).ok()?;

        for entry in dir {
            let entry = entry.ok()?;
            let entry_type = entry.path().join("type");

            if let Ok(contents) = std::fs::read_to_string(&entry_type)
            && contents.trim() == "Battery"
            {
                return Some(entry.path().join("uevent"));
            }
        }

        None
    }

    fn get_battery_level(&self) -> Option<f32> {
        let parsed_current_charge = self.current_charge.parse::<f32>().ok()?;
        let parsed_total_charge = self.total_charge.parse::<f32>().ok()?;

        if parsed_total_charge == 0.0 {
            return None;
        }

        let display_value = parsed_current_charge / parsed_total_charge * 100.0;

        Some(display_value)
    }

    fn set_battery_info(&mut self) -> Option<()> {
        // use battery_path and if not set find the battery
        // and set it
        if self.battery_path.is_none() {
            self.battery_path = BatteryInfo::find_battery();
        }
        let event_path = match &self.battery_path {
            Some(path) => path,
            None => {
                self.status = "POW".to_string();
                self.current_charge = 0.to_string();
                self.total_charge = 0.to_string();
                return Some(());
            }
        };
    
        // read from the event_path
        let content = std::fs::read_to_string(event_path).ok()?;

        for line in content.lines() {
            let split_str: Vec<&str> = line.split("=").collect();
            let key = split_str[0];
            let value = split_str[1];

            if key == POWER_SUPPLY_ENERGY_NOW {
                self.current_charge = value.to_string();
            }

            // full design gets us the full capacity of the battery
            // before degradation?
            // NOTE: this is what i3status uses
            if key == POWER_SUPPLY_ENERGY_FULL_DESIGN {
                self.total_charge = value.to_string();
            }

            if key == POWER_SUPPLY_STATUS && value == "Charging" {
                self.status = "CHR".to_string()
            }
        }

        Some(())
    }
}

pub fn get_batt_status_line() -> Vec<status::block::StatusLineBlock> {
    let mut battery_info = BatteryInfo::default();

    // set the values
    let _ = battery_info.set_battery_info();

    // return values
    let battery_level = battery_info.get_battery_level().unwrap_or(0.0);
    let status = battery_info.status;
    let icon: char = get_battery_icon(battery_level, status);
    let battery = status::block::StatusLineBlock {
        full_text: format!("{:.2}%", battery_level),
        short_text: format!("{:.2}%", battery_level),
        name: "bat".to_string(),
        instance: "bat".to_string(),
        separator: true,
        ..Default::default()
    };
    let battery_icon = status::block::StatusLineBlock {
        full_text: format!("<span size=\"{}\">{}</span>", 16 * 1024, icon),
        short_text: format!("{}", icon),
        color: get_battery_color(battery_level),
        ..Default::default()
    };
    vec![battery_icon, battery]
}

fn get_battery_icon(battery_level: f32, status: String) -> char {
    match status.as_str() {
        "POW" => POW,
        "CHR" => {
            if battery_level > 50.00 {
                FULL_CHG
            } else if battery_level > 25.00 {
                HALF_CHG
            } else {
                QUARTER_CHG
            }
        },
        _ => {
            if battery_level > 50.00 {
                FULL
            } else if battery_level > 25.00 {
                HALF
            } else {
                QUARTER
            }
        }
    }
}

fn get_battery_color(battery_level: f32) -> String {
    if battery_level > 50.00 {
        "#008000".to_string()
    } else if battery_level > 25.00 {
        "#ffce1b".to_string()
    } else {
        "#cd1c18".to_string()
    }
}
