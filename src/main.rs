use color_eyre::Result;
use crossterm::ExecutableCommand;
use ratatui;

use crate::app::App;

mod app;
mod colors; 

fn main() -> Result<()> {
    // Must explicitly enable mouse capture
    std::io::stdout().execute(crossterm::event::EnableMouseCapture)?;

    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let result = App::default().run(&mut terminal);
    ratatui::restore();

    // Must explicitly disable mouse capture
    std::io::stdout().execute(crossterm::event::DisableMouseCapture)?;
    result
}