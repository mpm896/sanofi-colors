use std::{cell::RefCell, io};

use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, MouseButton, MouseEvent, MouseEventKind};
use ratatui::{
    buffer::Buffer, layout::{Constraint, Flex, Layout, Rect}, style::{Style, Stylize}, symbols::border, text::{Line, Span}, widgets::{Block, Padding, Paragraph, Widget}, DefaultTerminal, Frame
};
use strum::IntoEnumIterator;

use crate::colors;

const VERTICAL_DIM: Constraint = Constraint::Ratio(1, 6);
const HORIZONTAL_DIM: Constraint = Constraint::Ratio(1, 3);
const TITLE: &'static str = "  Sanofi Colors  ";

#[derive(Debug, Default)]
pub struct App {
    exit: bool,
    state: RefCell<State>  // Allows interior mutability
}

impl App {

    /// Runs the app's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let grid:  Grid = Grid { cols: 2, rows: 3 };
        let layout = grid.compute_layout(frame.area());
        self.state.borrow_mut().color_areas = layout;

        frame.render_widget(self, frame.area());
        frame.render_widget(grid, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            Event::Mouse(mouse_event) => { //if mouse_event.kind == MouseEventKind::Down(MouseButton::Left) => {
                self.handle_mouse_click(mouse_event);
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn handle_mouse_click(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Down(button) if button == MouseButton::Left => {
                if let Some(c) = self.get_color_state(mouse_event) {
                    self.set_color(c);
                }        
            },
            _ => {}
        }
    }

    fn set_color(&mut self, c: colors::Colors) {
        self.state.borrow_mut().selected = Some(c);
    }

    fn get_color_state(&self, mouse_event: MouseEvent) -> Option<colors::Colors> {
        let x = mouse_event.column;
        let y: u16 = mouse_event.row;

        for (color, area) in &self.state.borrow().color_areas {
            if Block::new().inner(*area).contains((x, y).into()) {
                return Some(*color)
            }
        }
        None
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(TITLE.bold());
        let instructions: Line<'_> = if let Some(c) = self.state.borrow().selected {
            Line::from(vec![
                Span::raw(" "),
                format!(" {} ", c)
                    .fg(colors::Colors::MiraclePurple.get_color())
                    .bg(colors::Colors::SupportGray.get_color()),
                Span::raw(" ")
            ])
        } else {
            Line::from(vec![
                Span::raw(" "),
                " Choose a color to copy the hex code "
                    .fg(colors::Colors::MiraclePurple.get_color())
                    .bg(colors::Colors::SupportGray.get_color()),
                    Span::raw(" ")
            ])
        };

        // Render a block with top and bottom title and borders for the terminal screen
        Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .padding(Padding::new( // Center vertically
                0, 0, area.height / 2, 0
            ))
            .border_set(border::ROUNDED)
            .render(area, buf);
    }
}

#[derive(Debug, Default)]
struct State {
    selected: Option<colors::Colors>,
    color_areas: Vec<(colors::Colors, Rect)>
}


struct Grid {
    cols: usize,
    rows: usize
}

impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let color_layout: Vec<(colors::Colors, Rect)> = self.compute_layout(area);
        for (color, cell) in color_layout {
            let color_style = color.get_color();
            
            let h = cell.height;
            let [area] = Layout::vertical([Constraint::Length(h)])
                .flex(Flex::Center)
                .areas(cell);

            let title: Line<'_> = Line::from(format!("  {}  ", color))
                .fg(color.get_color())
                .centered();
            
            let block: Paragraph<'_> = Paragraph::new("")
            .block(Block::bordered()
                .border_set(border::ROUNDED)
                .title(title))
            .centered();

            let inner = Block::bordered().inner(cell);
            let inner_paragraph = Paragraph::new("")
                .style(Style::default().bg(color_style));

            block.render(area, buf);
            inner_paragraph.render(inner, buf);
        }
    }
}

impl Grid {
    fn compute_layout(&self, area: Rect) -> Vec<(colors::Colors, Rect)> {
        let col_constraints = (0..self.cols).map(|_| HORIZONTAL_DIM);
        let row_constrains = (0..self.rows).map(|_| VERTICAL_DIM);
        let horizontal = Layout::horizontal(col_constraints).spacing(1).flex(Flex::Center);
        let vertical = Layout::vertical(row_constrains).spacing(3).flex(Flex::Center);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());
        
        let colors = colors::Colors::iter();
        let color_layout: Vec<(colors::Colors, Rect)> = colors.zip(cells).collect();
        color_layout
    }
}



#[cfg(test)]
mod tests {
    use std::io;
    use crossterm::event::KeyCode;

    use crate::App;

    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into());
        assert!(app.exit);

        Ok(())
    }
}