mod cpu;
mod pc;
mod register;

pub use cpu::Cpu as RiscV;
use pc::PC;
pub use register::Registers;
