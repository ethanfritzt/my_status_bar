#[derive(Default, Debug, serde::Serialize, serde::Deserialize)]
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
