#[derive(Clone)]
pub struct BColors {
    pub header: String,
    pub blue: String,
    pub cyan: String,
    pub cyan_green: String,
    pub warning: String,
    pub fail: String,
    pub endc: String,
    pub bold: String,
    pub underline: String,
    pub purple: String,
    pub orange: String,
    pub yellow: String,
    pub green: String, 
}

impl BColors {
    pub fn new() -> Self {
        BColors {
            header: "\x1b[95m".to_string(),
            blue: "\x1b[94m".to_string(),
            cyan: "\x1b[96m".to_string(),
            cyan_green: "\x1b[92m".to_string(),
            warning: "\x1b[93m".to_string(),
            fail: "\x1b[91m".to_string(),
            endc: "\x1b[0m".to_string(),
            bold: "\x1b[1m".to_string(),
            underline: "\x1b[4m".to_string(),
            purple: "\x1b[35m".to_string(),
            orange: "\x1b[33m".to_string(),
            yellow: "\x1b[93m".to_string(),
            green: "\x1b[32m".to_string(), 
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ReservationStation {
    pub busy: bool,
    pub op: Option<String>,
    pub vj: Option<i32>,
    pub vk: Option<i32>,
    pub qj: Option<usize>,
    pub qk: Option<usize>,
    pub result: Option<i32>,
}

#[derive(Default, Clone, Debug)]
pub struct Instruction {
    pub op: String,
    pub dest: String,
    pub src1: String,
    pub src2: Option<String>,
}

#[derive(Default, Clone, Debug)]
pub struct RegisterStatus {
    pub value: i32,
    pub reservation_station: Option<usize>,
}