use risc_v_emulator::riscv::RiscV;
use risc_v_emulator::riscv::loader;

fn main() {
    let mut machine = RiscV::new();
    
    match loader::read_binary("test") {
        Ok(code) => {
            if let Err(e) = machine.cycle(&code) {
                eprintln!("Error: {}", e);
            }

            machine.print();
        },
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}