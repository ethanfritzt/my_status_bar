#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct StatusLineBlock { 
    pub full_text: String,
    pub short_text: String,
    pub color: String,
    pub background: String,
    pub border: String,
    pub border_top: i32,
    pub border_bottom: i32,
    pub border_left: i32,
    pub border_right: i32,
    pub min_width: i32,
    pub align: String,
    pub name: String,
    pub instance: String,
    pub urgent: bool,
    pub separator: bool,
    pub separator_block_width: i32,
    pub markup: String
}

impl Default for StatusLineBlock {
    fn default () -> Self {
       Self {
        full_text: "".to_string(), 
        short_text: "".to_string(),
        color: "#fffffff".to_string(),
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
       } 
    }
} 
