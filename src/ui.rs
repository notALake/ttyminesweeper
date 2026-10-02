use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Modifier},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use crate::app::{App, GameState, Selected};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)])
        .split(frame.area());

    let borders = Block::default().borders(Borders::ALL);
    
    //Matrix Dimension Gui Selector -- the app.map width and height are being set in main.rs, with the KeyCode press
    if let GameState::Menu = app.state { 
        frame.render_widget(&borders, center_area(chunks[1], frame.area().width-4, frame.area().height-4));
        frame.render_widget(Paragraph::new(format!("9x9")), ratatui::layout::Rect{x: frame.area().width/4-1, y: frame.area().height/2-3, width: 3, height: 1});
        frame.render_widget(Paragraph::new(format!("30x16")), ratatui::layout::Rect{x: frame.area().width/2-2, y: frame.area().height/2-3, width: 5, height: 1});
        frame.render_widget(Paragraph::new(format!("Custom")), ratatui::layout::Rect{x: frame.area().width*3/4-3, y: frame.area().height/2-3, width: 6, height: 1});
        frame.render_widget(Paragraph::new(format!("Width: {}", app.map_width)), ratatui::layout::Rect{x: frame.area().width/3-5, y: frame.area().height/2+2, width: 10, height: 1});
        frame.render_widget(Paragraph::new(format!("Height: {}", app.map_height)), ratatui::layout::Rect{x: frame.area().width*2/3-5, y: frame.area().height/2+2, width: 11, height: 1});
        frame.render_widget(Paragraph::new(format!("Press k, z or f to flag")), ratatui::layout::Rect{x: frame.area().width/2-10, y: frame.area().height/2+6, width: 23, height: 1});
        frame.render_widget(Paragraph::new(format!("Press Space, Enter, x or l to open a cell")), ratatui::layout::Rect{x: frame.area().width/2-20, y: frame.area().height/2+7, width: 41, height: 1});
        frame.render_widget(Paragraph::new(format!("Press q to quit")), ratatui::layout::Rect{x: frame.area().width/2-6, y: frame.area().height/2+8, width: 15, height: 1});

        if let Selected::Profile1 = app.selection
        {
            frame.render_widget(Paragraph::new(format!("9x9")).style(Style::default().add_modifier(Modifier::REVERSED)), ratatui::layout::Rect{x: frame.area().width/4-1, y: frame.area().height/2-3, width: 3, height: 1});
        }
        else if let Selected::Profile2 = app.selection
        {
            frame.render_widget(Paragraph::new(format!("30x16")).style(Style::default().add_modifier(Modifier::REVERSED)), ratatui::layout::Rect{x: frame.area().width/2-2, y: frame.area().height/2-3, width: 5, height: 1});
        }
        else if let Selected::Width = app.selection
        {
            frame.render_widget(Paragraph::new(format!("Custom")).style(Style::default().add_modifier(Modifier::REVERSED)), ratatui::layout::Rect{x: frame.area().width*3/4-3, y: frame.area().height/2-3, width: 6, height: 1});
            frame.render_widget(Paragraph::new(format!("Width: {}", app.map_width)).style(Style::default().add_modifier(Modifier::REVERSED)), ratatui::layout::Rect{x: frame.area().width/3-5, y: frame.area().height/2+2, width: 10, height: 1});
        }
        else if let Selected::Height = app.selection
        {
            frame.render_widget(Paragraph::new(format!("Custom")).style(Style::default().add_modifier(Modifier::REVERSED)), ratatui::layout::Rect{x: frame.area().width*3/4-3, y: frame.area().height/2-3, width: 6, height: 1});
            frame.render_widget(Paragraph::new(format!("Height: {}", app.map_height)).style(Style::default().add_modifier(Modifier::REVERSED)), ratatui::layout::Rect{x: frame.area().width*2/3-5, y: frame.area().height/2+2, width: 11, height: 1});
        }
    }

    if let GameState::Playing = app.state { 
        frame.render_widget(&borders, center_area(chunks[0], frame.area().width, frame.area().height));
        frame.render_widget(Paragraph::new(format!("Timer: {}", app.timer)).style(Style::default().fg(Color::Rgb(0,255,0))), ratatui::layout::Rect{x: 6, y: 1, width: 13, height: 1});
        //VICTORY
        if app.start && app.ncells==0
        {
            frame.render_widget(Paragraph::new(format!("VICTORY")).style(Style::default().fg(Color::Rgb(0,0,255))), ratatui::layout::Rect{x: frame.area().width/2-4, y: 1, width: 10, height: 1});
            frame.render_widget(Paragraph::new(format!("Press Enter to Continue")).style(Style::default().fg(Color::Rgb(255,255,255))), ratatui::layout::Rect{x: frame.area().width/2-12, y: 2, width: 23, height: 1});
        }
        //DEFEAT
        else if app.defeat
        {
            frame.render_widget(Paragraph::new(format!("DEFEAT")).style(Style::default().fg(Color::Rgb(255,0,0))), ratatui::layout::Rect{x: frame.area().width/2-3, y: 1, width: 10, height: 1});
            frame.render_widget(Paragraph::new(format!("Press Enter to Continue")).style(Style::default().fg(Color::Rgb(255,255,255))), ratatui::layout::Rect{x: frame.area().width/2-12, y: 2, width: 23, height: 1});
        }
        else
        {
            frame.render_widget(Paragraph::new(format!("Minesweeper")).style(Style::default().fg(Color::Rgb(255,255,255))), ratatui::layout::Rect{x: frame.area().width/2-4, y: 1, width: 15, height: 1});
        }
        frame.render_widget(Paragraph::new(format!("Flags: {}", app.bombs)).style(Style::default().fg(Color::Rgb(255,0,0))), ratatui::layout::Rect{x: frame.area().width-15, y: 1, width: 13, height: 1});
        let game_area = center_area(chunks[1], app.map_width as u16, app.map_height as u16);
        frame.render_widget(&borders, game_area);
        for k in 1..=app.map_height.saturating_sub(2)
        {
            for j in 1..=app.map_width.saturating_sub(2)
            {
                let mut num = ' ';
                let mut style = Style::default().bg(Color::Rgb(1,1,1));
                let num_rect = ratatui::layout::Rect{x: game_area.x + j as u16, y: game_area.y + k as u16, width: 1, height: 1,};
                if app.clicked[k*app.map_width+j]==1
                { 
                    (num, style) = match app.located[k*app.map_width+j]
                    {
                        9 => ('®', Style::default().fg(Color::Rgb(255,255,255)).bg(Color::Rgb(1,1,1))),
                        8 => ('8', Style::default().fg(Color::Rgb(255,0,0)).bg(Color::Rgb(64,64,64))),
                        7 => ('7', Style::default().fg(Color::Rgb(255,50,0)).bg(Color::Rgb(64,64,64))),
                        6 => ('6', Style::default().fg(Color::Rgb(255,100,0)).bg(Color::Rgb(64,64,64))),
                        5 => ('5', Style::default().fg(Color::Rgb(255,180,0)).bg(Color::Rgb(64,64,64))),
                        4 => ('4', Style::default().fg(Color::Rgb(255,255,0)).bg(Color::Rgb(64,64,64))),
                        3 => ('3', Style::default().fg(Color::Rgb(170,255,0)).bg(Color::Rgb(64,64,64))),
                        2 => ('2', Style::default().fg(Color::Rgb(80,255,0)).bg(Color::Rgb(64,64,64))),
                        1 => ('1', Style::default().fg(Color::Rgb(0,255,0)).bg(Color::Rgb(64,64,64))),
                        _ => (' ', Style::default().bg(Color::Rgb(64,64,64))),
                    };
                }
                else if app.clicked[j+k*app.map_width]==2
                {
                    num = 'F';
                    style = Style::default().fg(Color::Rgb(255, 0, 0)).bg(Color::Rgb(1,1,1));
                }
                if j==app.cursor_x && k==app.cursor_y {style = style.add_modifier(Modifier::REVERSED);}
                if !(app.map_width as u16>frame.area().width || app.map_height as u16>frame.area().height) {frame.buffer_mut()[(num_rect.x, num_rect.y)].set_char(num).set_style(style);}
            }
        }
    }

    fn center_area(area: Rect, width: u16, height: u16) -> Rect {
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(height),
                Constraint::Fill(1),
            ])
            .split(area);

        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Fill(1),
                Constraint::Length(width),
                Constraint::Fill(1),
            ])
            .split(vertical[1])[1]
    }
}
