use crate::settings::BColors;
use lazy_static::lazy_static;

lazy_static! {
    static ref COLORS: BColors = BColors::new();
}

pub fn log_instruction(instruction: &str) {
    println!(
        "{}[DYNSCHED]{} {}[INSTRUCTION]{} {}",
        COLORS.cyan_green, COLORS.endc,
        COLORS.purple, COLORS.endc,
        instruction
    );
}

pub fn log_reservation_station_update(update: &str) {
    println!(
        "{}[DYNSCHED]{} {}[RESERVATION STATION UPDATE]{} {}",
        COLORS.cyan_green, COLORS.endc,
        COLORS.orange, COLORS.endc,
        update
    );
}

pub fn log_execution_attempt(attempt: &str) {
    println!(
        "{}[DYNSCHED]{} {}[EXECUTION ATTEMPT]{} {}",
        COLORS.cyan_green, COLORS.endc,
        COLORS.yellow, COLORS.endc,
        attempt
    );
}

pub fn log_execution_result(result: &str) {
    println!(
        "{}[DYNSCHED]{} {}[EXECUTION RESULT]{} {}",
        COLORS.cyan_green, COLORS.endc,
        COLORS.green, COLORS.endc,
        result
    );
}

pub fn log_cdb_write(write: &str) {
    println!(
        "{}[DYNSCHED]{} {}[CDB WRITE]{} {}",
        COLORS.cyan_green, COLORS.endc,
        COLORS.blue, COLORS.endc,
        write
    );
}
