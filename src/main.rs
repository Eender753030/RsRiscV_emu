use risc_v_emulator::riscv::RiscV;
use risc_v_emulator::riscv::{loader, parser};
use risc_v_emulator::ui::{self, state};

use anyhow::Result;

// Main entry for Risc-V emulator. Return any errors.
fn main() -> Result<()> {
    let mut ins_list = vec![];
    let mut machine = RiscV::default();

    // Access binary data and load instructions into Risc-V's instruction memory
    for arg in loader::load_arg()? {
        let code = loader::read_binary(&arg)?;
        machine.load(&code)?;
        // Parse the binary and turn into &str for display instructions
        ins_list.extend(parser::parse_binary(code));
    }

    // Dump initial state of Risc-V
    let (reg_data, mem_data, pc_num) = machine.dump();

    // Create mut instant for Risc-V's state. Mut is for changing data and state of Risc-V
    let mut emu_state = state::EmuState::new(ins_list, reg_data, mem_data, pc_num);

    // Go into the TUI display loop
    ui::tui_loop(&mut emu_state, &mut machine)?;

    Ok(())
}