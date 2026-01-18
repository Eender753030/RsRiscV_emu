mod cpu;
mod pc;
mod register;

use pc::PC;
pub use register::Registers;
pub use cpu::Cpu as RiscV;