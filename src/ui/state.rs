use ratatui::widgets::ListState;

pub enum Selected {
    Ins,
    Reg,
    Mem,
}

pub enum EmuMode {
    Observation,
    Stay,
    Running,
}

pub struct ListStateRecord<T> {
    pub list: Vec<T>,
    pub list_state: ListState,
    current_select: usize,
}

pub struct EmuState {
    pub ins: ListStateRecord<String>,
    pub reg: ListStateRecord<i32>,
    pub mem: ListStateRecord<[u8; 4]>,
    pub pc: u32,
    pub mode: EmuMode,
    pub selected: Selected,
}

impl EmuState {
    pub fn new(ins_list: Vec<String>, reg_data: Vec<i32>, mem_data: Vec<[u8; 4]>, pc_num: u32) -> Self {
        let mut ins: ListStateRecord<String> = ListStateRecord {list: ins_list, list_state: ListState::default(), current_select: 0}; 
        let mut reg: ListStateRecord<i32> = ListStateRecord {list: reg_data, list_state: ListState::default(), current_select: 0}; 
        let mut mem: ListStateRecord<[u8; 4]> = ListStateRecord {list: mem_data, list_state: ListState::default(), current_select: 0}; 
        let pc = pc_num;
        let mode = EmuMode::Observation;
        let selected = Selected::Ins;

        ins.list_state.select(Some(0));
        reg.list_state.select(None);
        mem.list_state.select(None);
        
        EmuState {ins, reg, mem, pc, mode, selected}
    }

    pub fn update_data(&mut self, data_tuple: (Vec<i32>, Vec<[u8; 4]>, u32)) {
        (self.reg.list, self.mem.list, self.pc) = data_tuple;
    }

    pub fn update_ins_selected(&mut self) {
        if self.pc == 0 {
            self.ins.list_state.select(None);
        } else {
            self.ins.list_state.select(Some((self.pc / 4 - 1) as usize));
        }
    }

    pub fn running_mode_selected(&mut self) {
        match self.selected {
            Selected::Ins => {
                if self.pc == 0 {
                    self.ins.list_state.select(None);
                } else {
                    self.ins.list_state.select(Some((self.pc / 4 - 1) as usize));
                }
            },
            Selected::Reg => self.reg.list_state.select(None),
            Selected::Mem => self.mem.list_state.select(None)
        }
    }

    pub fn observation_mode_selected(&mut self) {
        match self.selected {
            Selected::Ins => self.ins.list_state.select(Some(self.ins.current_select)),
            Selected::Reg => self.reg.list_state.select(Some(self.reg.current_select)),
            Selected::Mem => self.mem.list_state.select(Some(self.mem.current_select))
        }
    }

    pub fn go_left(&mut self) {  
        self.selected = match self.selected {
            Selected::Ins => {
                self.ins.list_state.select(None);
                self.mem.list_state.select(Some(self.mem.current_select));
                Selected::Mem
            }
            Selected::Reg => {
                self.reg.list_state.select(None);
                self.ins.list_state.select(Some(self.ins.current_select));
                Selected::Ins
            }
            Selected::Mem => {
                self.mem.list_state.select(None);
                self.reg.list_state.select(Some(self.reg.current_select));
                Selected::Reg
            }
        };
    }

    pub fn go_right(&mut self) {  
        self.selected = match self.selected {
            Selected::Ins => {
                self.ins.list_state.select(None);
                self.reg.list_state.select(Some(self.reg.current_select));
                Selected::Reg
            }
            Selected::Reg => {
                self.reg.list_state.select(None);
                self.mem.list_state.select(Some(self.mem.current_select));
                Selected::Mem
            }
            Selected::Mem => {
                self.mem.list_state.select(None);
                self.ins.list_state.select(Some(self.ins.current_select));
                Selected::Ins
            }
        };
    }

    pub fn next(&mut self) {  
        match self.selected {
            Selected::Ins => {
                self.ins.current_select = match self.ins.current_select >= self.ins.list.len() - 1 {
                    true => 0,
                    false => self.ins.current_select + 1
                };
                self.ins.list_state.select(Some(self.ins.current_select));
            },
            Selected::Reg => {
                self.reg.current_select = match self.reg.current_select >= self.reg.list.len() - 1 {
                    true => 0,
                    false => self.reg.current_select + 1
                };
                self.reg.list_state.select(Some(self.reg.current_select));
            },
            Selected::Mem => {
                self.mem.current_select = match self.mem.current_select >= self.mem.list.len() - 1 {
                    true => 0,
                    false => self.mem.current_select + 1
                };
                self.mem.list_state.select(Some(self.mem.current_select));
            },
        }
    }
    
    pub fn prev(&mut self) {
        match self.selected {
            Selected::Ins => {
                self.ins.current_select = match self.ins.current_select == 0 {
                    true => self.ins.list.len() - 1,
                    false => self.ins.current_select - 1
                };
                self.ins.list_state.select(Some(self.ins.current_select));
            },
            Selected::Reg => {
                self.reg.current_select = match self.reg.current_select == 0  {
                    true => self.reg.list.len() - 1,
                    false => self.reg.current_select - 1
                };
                self.reg.list_state.select(Some(self.reg.current_select));
            },
            Selected::Mem => {
                self.mem.current_select = match self.mem.current_select == 0  {
                    true => self.mem.list.len() - 1,
                    false => self.mem.current_select - 1
                };
                self.mem.list_state.select(Some(self.mem.current_select));
            },
        }
    }
}