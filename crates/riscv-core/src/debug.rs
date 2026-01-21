pub struct MachineState {
    pub pc: u32,
    pub regs: [u32; 32],
    pub csrs: Vec<(String, u32)>,
}

pub trait DebugInterface {
    fn inspect_regs(&self) -> [u32; 32];

    fn inspect_pc(&self) -> u32;

    fn inspect_csrs(&self) -> Vec<(String, u32)>;

    fn inspect_ins(&self, start: u32, count: usize) -> Vec<(u32, String)>;

    fn inspect_mem(&self, start: u32, len: usize) -> Vec<u8>;
}

impl MachineState {
    pub fn update(machine: &dyn DebugInterface) -> MachineState {
        MachineState {
            pc: machine.inspect_pc(),
            regs: machine.inspect_regs(),
            csrs: machine.inspect_csrs(),
        }
    }
}