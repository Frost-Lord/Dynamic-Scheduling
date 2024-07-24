#[derive(Clone)]
pub struct BColors {
    pub blue: String,
    pub cyan_green: String,
    pub endc: String,
    pub purple: String,
    pub orange: String,
    pub yellow: String,
    pub green: String, 
}

impl BColors {
    pub fn new() -> Self {
        BColors {
            blue: "\x1b[94m".to_string(),
            cyan_green: "\x1b[92m".to_string(),
            endc: "\x1b[0m".to_string(),
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