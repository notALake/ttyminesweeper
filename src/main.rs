extern crate ratatui;
extern crate crossterm;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;
use std::{io::{stdout, Result}, time::{Duration, Instant}};

mod app;
mod ui;

use app::App;
use app::GameState;
use app::Selected;

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();
    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    while !app.should_quit{
        terminal.draw(|frame| ui::render(frame, &app))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));
        if event::poll(timeout)? {
            if let GameState::Playing = app.state
            {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press 
                    {
                        if !app.defeat
                        {
                            match key.code {
                                KeyCode::Char('q') => app.should_quit = true,
                                KeyCode::Left | KeyCode::Char('a') => app.move_cursor(-1, 0),
                                KeyCode::Right | KeyCode::Char('d') => app.move_cursor(1, 0),
                                KeyCode::Up | KeyCode::Char('w') => app.move_cursor(0, -1),
                                KeyCode::Down | KeyCode::Char('s') => app.move_cursor(0, 1),
                                KeyCode::Char('k') | KeyCode::Char('z') | KeyCode::Char('f')=> app.click2(),
                                KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('l') | KeyCode::Char('x') => {
                                    //Generate if the game ain't started yet 
                                    if !app.start {app.start=true; app.generate();}
                                    app.click();
                                },
                                _ => {}
                            }
                        }
                        else
                        {
                            match key.code {
                                KeyCode::Char('q') => app.should_quit=true,
                                KeyCode::Enter => {
                                    app.state = GameState::Menu;
                                    app.reset();
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
            else if let GameState::Menu = app.state
            {
                if let Event::Key(key) = event::read()?
                {
                    if key.kind == KeyEventKind::Press 
                    {
                        match key.code
                        {
                            KeyCode::Char('q') => app.should_quit = true,
                            KeyCode::Enter => {
                                //Dynamic bombs number and cursor location, to avoid impossible situation
                                app.bombs = app.map_width*app.map_height*13/100;
                                app.cursor_x = app.map_width-app.map_width/2;
                                app.cursor_y = app.map_height-app.map_height/2;
                                app.state = GameState::Playing;
                            }
                            KeyCode::Up | KeyCode::Char('w') => {
                                match app.selection {
                                    Selected::Width => app.map_width +=1,
                                    Selected::Height => app.map_height+=1,
                                    _ => {}
                                }
                            },
                            KeyCode::Down | KeyCode::Char('s') => {
                                match app.selection {
                                    //Avoiding freeze due to impossibile map generation(and cursor outside the map)
                                    Selected::Width => if app.map_width*app.map_height > 45 && app.map_width>4 {app.map_width-=1;},
                                    Selected::Height => if app.map_width*app.map_height > 45 && app.map_height>4 {app.map_height-=1;},
                                    _ => app.selection = Selected::Width,
                                }
                            },
                            KeyCode::Right | KeyCode::Char('d') => {
                                match app.selection {
                                    Selected::Profile1 => {
                                        app.selection = Selected::Profile2;
                                        app.map_width=30;
                                        app.map_height=16;
                                    }
                                    Selected::Profile2 => app.selection = Selected::Width,
                                    Selected::Width => app.selection = Selected::Height,
                                    Selected::Height => {
                                        app.selection = Selected::Profile1;
                                        app.map_width=9;
                                        app.map_height=9;
                                    }
                                }
                            },
                            KeyCode::Left | KeyCode::Char('a') => {
                                match app.selection {
                                    Selected::Profile2 => {
                                        app.selection = Selected::Profile1;
                                        app.map_width=9;
                                        app.map_height=9;
                                    }
                                    Selected::Width => {
                                        app.selection = Selected::Profile2;
                                        app.map_width=30;
                                        app.map_height=16;
                                    }
                                    Selected::Height => app.selection = Selected::Width,
                                    Selected::Profile1 => app.selection = Selected::Height,
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
