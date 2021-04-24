extern crate warp;
extern crate reqwest;

use serde::{ Deserialize, Serialize };
use warp::Filter;

#[derive(Deserialize, Serialize)]
struct CpuFlags {
    sign: bool,
    zero: bool,
    #[serde(rename = "auxCarry")] aux_carry: bool,
    parity: bool,
    carry: bool
}

#[derive(Deserialize, Serialize)]
struct CpuState {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    #[serde(rename = "stackPointer")] stack_pointer: u16,
    #[serde(rename = "programCounter")] program_counter: u16,
    cycles: u64,
    flags: CpuFlags
}

#[derive(Deserialize, Serialize)]
struct Cpu {
    state: CpuState,
    id: String,
    opcode: u8
}

async fn execute(mut cpu: Cpu) -> Result<impl warp::Reply, warp::Rejection> {
    cpu.state.cycles = cpu.state.cycles.wrapping_add(4);

    let msn = cpu.state.a >> 4;
    let lsn = cpu.state.a & 0b0000_1111;

    let mut correction = if cpu.state.flags.aux_carry || (lsn > 9) { 0x06 } else { 0x0 };
    if cpu.state.flags.carry || (msn > 9) || (msn == 9 && lsn > 9) { 
        correction += 0x60;
        cpu.state.flags.carry = true;
    }
    
    let result = cpu.state.a.wrapping_add(correction);
    cpu.state.flags.sign = (result & 0b1000_0000) == 0b1000_0000;
    cpu.state.flags.zero = result == 0;
    cpu.state.flags.aux_carry = (cpu.state.a & 0x0f) + (correction & 0x0f) > 0x0F;
    cpu.state.flags.parity = (result.count_ones() & 0b1) == 0;
    cpu.state.a = result;

    Ok(warp::reply::json(&cpu))
}

#[tokio::main]
async fn main() {
    let status = warp::get()
        .and(warp::path!("status"))
        .map(|| {
            "Healthy"
        });

    let execute = warp::post()
        .and(warp::path!("api"/"v1"/"execute"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and_then(execute);

    warp::serve(execute.or(status)).run(([0, 0, 0, 0], 8080)).await
}