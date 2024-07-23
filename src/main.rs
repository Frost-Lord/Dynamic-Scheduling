use std::collections::HashMap;
use crate::settings::{ReservationStation, Instruction, RegisterStatus};
mod settings;
mod logs;
use logs::*;

impl RegisterStatus {
    fn new(value: i32) -> Self {
        RegisterStatus {
            value,
            reservation_station: None,
        }
    }
}

fn main() {
    let mut reservation_stations: Vec<ReservationStation> = vec![Default::default(); 5];
    let mut register_status: HashMap<String, RegisterStatus> = HashMap::new();
    for i in 0..10 {
        register_status.insert(format!("F{}", i), RegisterStatus::new(0));
    }
    let mut instruction_queue: Vec<Instruction> = vec![
        Instruction {
            op: "LOAD".to_string(),
            dest: "F6".to_string(),
            src1: "R2".to_string(),
            src2: None,
        },
        Instruction {
            op: "LOAD".to_string(),
            dest: "F2".to_string(),
            src1: "R3".to_string(),
            src2: None,
        },
        Instruction {
            op: "MULT".to_string(),
            dest: "F0".to_string(),
            src1: "F2".to_string(),
            src2: Some("F4".to_string()),
        },
        Instruction {
            op: "SUB".to_string(),
            dest: "F8".to_string(),
            src1: "F6".to_string(),
            src2: Some("F2".to_string()),
        },
        Instruction {
            op: "DIV".to_string(),
            dest: "F10".to_string(),
            src1: "F0".to_string(),
            src2: Some("F6".to_string()),
        },
        Instruction {
            op: "ADD".to_string(),
            dest: "F6".to_string(),
            src1: "F8".to_string(),
            src2: Some("F2".to_string()),
        },
    ];

    while !instruction_queue.is_empty() || reservation_stations.iter().any(|rs| rs.busy) {
        issue_instruction(&mut instruction_queue, &mut reservation_stations, &mut register_status);
        execute_instructions(&mut reservation_stations, &mut register_status);
    }

    for (reg, status) in &register_status {
        println!("{}: {}", reg, status.value);
    }
}

fn issue_instruction(
    instruction_queue: &mut Vec<Instruction>,
    reservation_stations: &mut Vec<ReservationStation>,
    register_status: &mut HashMap<String, RegisterStatus>,
) {
    if !instruction_queue.is_empty() {
        let instr = instruction_queue.remove(0);
        log_instruction(&format!("Issuing instruction: {:?}", instr));
        for (i, rs) in reservation_stations.iter_mut().enumerate() {
            if !rs.busy {
                rs.busy = true;
                rs.op = Some(instr.op.clone());
                rs.result = None; // Reset the result

                if let Some(qj) = register_status.get(&instr.src1).and_then(|status| status.reservation_station) {
                    rs.qj = Some(qj);
                    rs.vj = None; // Reset value if waiting for a result
                } else {
                    rs.vj = register_status.get(&instr.src1).map(|status| status.value);
                    rs.qj = None;
                }

                if let Some(src2) = &instr.src2 {
                    if let Some(qk) = register_status.get(src2).and_then(|status| status.reservation_station) {
                        rs.qk = Some(qk);
                        rs.vk = None; // Reset value if waiting for a result
                    } else {
                        rs.vk = register_status.get(src2).map(|status| status.value);
                        rs.qk = None;
                    }
                } else {
                    rs.vk = None;
                    rs.qk = None;
                }

                if let Some(dest_status) = register_status.get_mut(&instr.dest) {
                    dest_status.reservation_station = Some(i);
                }

                log_reservation_station_update(&format!("{:?}", rs));
                break;
            }
        }
    }
}

fn execute_instructions(
    reservation_stations: &mut Vec<ReservationStation>,
    register_status: &mut HashMap<String, RegisterStatus>,
) {
    let mut results_to_write = vec![];
    for (i, rs) in reservation_stations.iter_mut().enumerate() {
        if rs.busy {
            log_execution_attempt(&format!("{:?}", rs));
            if (rs.vj.is_some() || rs.qj.is_none())
                && (rs.vk.is_some() || rs.op.as_deref() == Some("LOAD") || rs.qk.is_none())
            {
                match rs.op.as_deref() {
                    Some("LOAD") => {
                        rs.result = Some(42); // Placeholder for actual LOAD operation
                    }
                    Some("MULT") => {
                        rs.result = rs.vj.and_then(|vj| rs.vk.map(|vk| vj * vk));
                    }
                    Some("SUB") => {
                        rs.result = rs.vj.and_then(|vj| rs.vk.map(|vk| vj - vk));
                    }
                    Some("DIV") => {
                        rs.result = rs.vj.and_then(|vj| rs.vk.map(|vk| vj / vk));
                    }
                    Some("ADD") => {
                        rs.result = rs.vj.and_then(|vj| rs.vk.map(|vk| vj + vk));
                    }
                    _ => {}
                }

                log_execution_result(&format!("{:?}", rs.result));
                if let Some(result) = rs.result {
                    results_to_write.push((i, result));
                }
            } else {
                log_execution_attempt(&format!("Reservation station not ready for execution: {:?}", rs));
            }
        }
    }

    for (index, result) in results_to_write {
        write_result_to_cdb(index, register_status, reservation_stations, result);
    }
}

fn write_result_to_cdb(
    rs_index: usize,
    register_status: &mut HashMap<String, RegisterStatus>,
    reservation_stations: &mut Vec<ReservationStation>,
    result: i32,
) {
    if let Some(ref mut dest_status) = register_status.values_mut().find(|status| status.reservation_station == Some(rs_index)) {
        dest_status.value = result;
        dest_status.reservation_station = None;
    }

    for station in reservation_stations.iter_mut() {
        if station.qj == Some(rs_index) {
            station.vj = Some(result);
            station.qj = None;
        }
        if station.qk == Some(rs_index) {
            station.vk = Some(result);
            station.qk = None;
        }
    }

    let rs = &mut reservation_stations[rs_index];
    log_cdb_write(&format!("{:?}", rs));
    rs.busy = false;
    rs.op = None;
    rs.vj = None;
    rs.vk = None;
    rs.qj = None;
    rs.qk = None;
    rs.result = None;
}
