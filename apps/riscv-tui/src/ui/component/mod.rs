mod instruction;
mod exception;
mod memory;
mod mid;

use ratatui::{
    Frame, 
    layout::Rect,
    style::{Color, Style}, 
    widgets::{Block, List, ListItem, ListState},
};

use crate::state::EmuState;

pub use instruction::Instruction;
pub use exception::Exception;
pub use memory::Memory;
#[cfg(feature = "zicsr")]
pub use mid::csr::Csr;
pub use mid::register::Register;

pub(super) const ANTI_FLASH_WHITE: Color = Color::Rgb(242, 242, 242);
pub(super) const BERKELEY_BLUE: Color = Color::Rgb(0, 50, 98);
pub(super) const CALIFORNIA_GOLD: Color = Color::Rgb(253, 181, 21);

pub trait Component {
    fn render(f: &mut Frame, area: Rect, emu: &mut EmuState);

    fn render_list_state(
        f: &mut Frame, 
        area: Rect, 
        items: Vec<ListItem>,
        state: &mut ListState, 
        title: &str
    ) {
        let list = List::new(items)
            .block(Block::bordered().title(title))
            .style(Style::default().bg(BERKELEY_BLUE).fg(CALIFORNIA_GOLD))
            .highlight_style(Style::default().bg(ANTI_FLASH_WHITE).fg(BERKELEY_BLUE));

        f.render_stateful_widget(list, area, state);
    }
}
