extern crate ratatui;
extern crate crossterm;

use crossterm::{event::{self, Event, KeyCode, KeyEventKind}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand,};
use ratatui::prelude::*;
use std::{io::{stdout, Result}, time::{Duration, Instant}};

mod app;
mod ui;

use crate::app::{App, GameState, SelectedMenu, SelectedMapSize, SelectedBombs};

fn main() -> Result<()>
{
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut app = App::new();
    let tick_rate = Duration::from_secs(1);
    let mut last_tick = Instant::now();
    while !app.should_quit
    {
        terminal.draw(|frame| ui::render(frame, &app))?;
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or(Duration::from_secs(0));
        if event::poll(timeout)?
        {
            if let GameState::Menu = app.state
            {
                if let Event::Key(key) = event::read()?
                {
                    if key.kind == KeyEventKind::Press
                    {
                        match key.code
                        {
                            KeyCode::Char('q') =>
                            {
                                if app.guide
                                {
                                    app.guide=false;
                                }
                                else
                                {
                                    app.should_quit = true;
                                }
                            },
                            KeyCode::Enter =>
                            {
                                if !app.guide
                                {
                                    match app.selection_menu
                                    {
                                        SelectedMenu::Play =>
                                        {
                                            app.cursor_x = app.map_width-app.map_width/2;
                                            app.cursor_y = app.map_height-app.map_height/2;
                                            app.located = vec![0; app.map_height*app.map_width];
                                            app.clicked = vec![0; app.map_height*app.map_width];
                                            app.state = GameState::Playing;
                                        },
                                        SelectedMenu::Guide => app.guide=true,
                                        SelectedMenu::GuessFree => app.guessfree=!app.guessfree,
                                        SelectedMenu::OptionsMap => app.state = GameState::MapSize,
                                        SelectedMenu::OptionsBomb => app.state = GameState::Bombs,
                                        SelectedMenu::Quit => app.should_quit = true,
                                    }
                                }
                            }
                            KeyCode::Up | KeyCode::Char('w') =>
                            {
                                if !app.guide
                                {
                                    match app.selection_menu
                                    {
                                        SelectedMenu::Play => app.selection_menu = SelectedMenu::Quit,
                                        SelectedMenu::Guide => app.selection_menu = SelectedMenu::Play,
                                        SelectedMenu::GuessFree => app.selection_menu = SelectedMenu::Guide,
                                        SelectedMenu::OptionsMap => app.selection_menu = SelectedMenu::GuessFree,
                                        SelectedMenu::OptionsBomb => app.selection_menu = SelectedMenu::OptionsMap,
                                        SelectedMenu::Quit => app.selection_menu = SelectedMenu::OptionsBomb,
                                    }
                                }
                            },
                            KeyCode::Down | KeyCode::Char('s') =>
                            {
                                if !app.guide
                                {
                                    match app.selection_menu
                                    {
                                        SelectedMenu::Play => app.selection_menu = SelectedMenu::Guide,
                                        SelectedMenu::Guide => app.selection_menu = SelectedMenu::GuessFree,
                                        SelectedMenu::GuessFree => app.selection_menu = SelectedMenu::OptionsMap,
                                        SelectedMenu::OptionsMap => app.selection_menu = SelectedMenu::OptionsBomb,
                                        SelectedMenu::OptionsBomb => app.selection_menu = SelectedMenu::Quit,
                                        SelectedMenu::Quit => app.selection_menu = SelectedMenu::Play,
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            else if let GameState::MapSize = app.state
            {
                if let Event::Key(key) = event::read()?
                {
                    if key.kind == KeyEventKind::Press 
                    {
                        match key.code
                        {
                            KeyCode::Enter | KeyCode::Char('q') | KeyCode::Backspace =>
                            {
                                app.state = GameState::Menu;
                            },
                            KeyCode::Up | KeyCode::Char('w') =>
                            {
                                match app.selection_map
                                {
                                    SelectedMapSize::Width => app.map_width +=1,
                                    SelectedMapSize::Height => app.map_height+=1,
                                    _ => {}
                                }
                            },
                            KeyCode::Down | KeyCode::Char('s') =>
                            {
                                match app.selection_map
                                {
                                    //Avoiding freeze due to impossibile map generation(and cursor outside the map)
                                    SelectedMapSize::Width => if app.map_width*app.map_height > 45 && app.map_width>4 {app.map_width-=1;},
                                    SelectedMapSize::Height => if app.map_width*app.map_height > 45 && app.map_height>4 {app.map_height-=1;},
                                    _ => app.selection_map = SelectedMapSize::Width,
                                }
                            },
                            KeyCode::Right | KeyCode::Char('d') =>
                            {
                                match app.selection_map
                                {
                                    SelectedMapSize::Profile1 =>
                                    {
                                        app.selection_map = SelectedMapSize::Profile2;
                                        app.map_width=30;
                                        app.map_height=16;
                                    }
                                    SelectedMapSize::Profile2 => app.selection_map = SelectedMapSize::Width,
                                    SelectedMapSize::Width => app.selection_map = SelectedMapSize::Height,
                                    SelectedMapSize::Height =>
                                    {
                                        app.selection_map = SelectedMapSize::Profile1;
                                        app.map_width=9;
                                        app.map_height=9;
                                    }
                                }
                            },
                            KeyCode::Left | KeyCode::Char('a') =>
                            {
                                match app.selection_map
                                {
                                    SelectedMapSize::Profile2 =>
                                    {
                                        app.selection_map = SelectedMapSize::Profile1;
                                        app.map_width=9;
                                        app.map_height=9;
                                    }
                                    SelectedMapSize::Width =>
                                    {
                                        app.selection_map = SelectedMapSize::Profile2;
                                        app.map_width=30;
                                        app.map_height=16;
                                    }
                                    SelectedMapSize::Height => app.selection_map = SelectedMapSize::Width,
                                    SelectedMapSize::Profile1 => app.selection_map = SelectedMapSize::Height,
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            else if let GameState::Bombs = app.state
            {
                if let Event::Key(key) = event::read()?
                {
                    if key.kind == KeyEventKind::Press
                    {
                        match key.code
                        {
                            KeyCode::Enter | KeyCode::Char('q') | KeyCode::Backspace =>
                            {
                                app.state = GameState::Menu;
                            },
                            KeyCode::Up | KeyCode::Char('w') =>
                            {
                                match app.selection_bombs
                                {
                                    SelectedBombs::Number =>
                                    {
                                        app.bombs +=1;
                                        app.bombs_percentual = app.bombs*100/app.map_height/app.map_width;
                                    },
                                    SelectedBombs::Percentual =>
                                    {
                                        app.bombs_percentual+=1;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                    _ => {}
                                }
                            },
                            KeyCode::Down | KeyCode::Char('s') =>
                            {
                                match app.selection_bombs
                                {
                                    SelectedBombs::Number =>
                                    {
                                        app.bombs -=1;
                                        app.bombs_percentual = app.bombs*100/app.map_height/app.map_width;
                                    },
                                    SelectedBombs::Percentual =>
                                    {
                                        app.bombs_percentual-=1;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                    _ => app.selection_bombs = SelectedBombs::Number,
                                }
                            },
                            KeyCode::Right | KeyCode::Char('d') =>
                            {
                                match app.selection_bombs
                                {
                                    SelectedBombs::Easy =>
                                    {
                                        app.selection_bombs = SelectedBombs::Medium;
                                        app.bombs_percentual = 16;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                    SelectedBombs::Medium =>
                                    {
                                        app.selection_bombs = SelectedBombs::Hard;
                                        app.bombs_percentual = 21;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                    SelectedBombs::Hard => app.selection_bombs = SelectedBombs::Number,
                                    SelectedBombs::Number => app.selection_bombs = SelectedBombs::Percentual,
                                    SelectedBombs::Percentual =>
                                    {
                                        app.selection_bombs = SelectedBombs::Easy;
                                        app.bombs_percentual = 13;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                }
                            },
                            KeyCode::Left | KeyCode::Char('a') =>
                            {
                                match app.selection_bombs
                                {
                                    SelectedBombs::Medium =>
                                    {
                                        app.selection_bombs = SelectedBombs::Easy;
                                        app.bombs_percentual = 13;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                    SelectedBombs::Easy => app.selection_bombs = SelectedBombs::Percentual,
                                    SelectedBombs::Percentual => app.selection_bombs = SelectedBombs::Number,
                                    SelectedBombs::Number =>
                                    {
                                        app.selection_bombs = SelectedBombs::Hard;
                                        app.bombs_percentual = 21;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                    SelectedBombs::Hard =>
                                    {
                                        app.selection_bombs = SelectedBombs::Medium;
                                        app.bombs_percentual = 16;
                                        app.bombs = app.map_height*app.map_width*app.bombs_percentual/100;
                                    },
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            else if let GameState::Playing = app.state
            {
                if let Event::Key(key) = event::read()?
                {
                    if key.kind == KeyEventKind::Press 
                    {
                        if !app.defeat
                        {
                            match key.code
                            {
                                KeyCode::Char('q') => app.should_quit = true,
                                KeyCode::Left | KeyCode::Char('a') => app.move_cursor(-1, 0),
                                KeyCode::Right | KeyCode::Char('d') => app.move_cursor(1, 0),
                                KeyCode::Up | KeyCode::Char('w') => app.move_cursor(0, -1),
                                KeyCode::Down | KeyCode::Char('s') => app.move_cursor(0, 1),
                                KeyCode::Char('k') | KeyCode::Char('z') | KeyCode::Char('f') => app.click2(),
                                KeyCode::Char(' ') | KeyCode::Enter | KeyCode::Char('l') | KeyCode::Char('x') =>
                                {
                                    //Generate if the game ain't started yet 
                                    if !app.start {app.start=true; app.generate();}
                                    app.click();
                                },
                                _ => {}
                            }
                        }
                        else
                        {
                            match key.code
                            {
                                KeyCode::Char('q') => app.should_quit=true,
                                KeyCode::Enter =>
                                {
                                    app.state = GameState::Menu;
                                    app.reset();
                                },
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        if last_tick.elapsed() >= tick_rate
        {
            app.on_tick();
            last_tick = Instant::now();
        }
    }
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
