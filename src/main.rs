use std::{
    io::{self, Stdout, Write},
    thread,
    time::{self, Duration},
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    execute, queue,
    style::{self, Color, ContentStyle, StyledContent},
    terminal::{self, ClearType},
};

fn has_coordinates(coords: (u16, u16), points: &[((u16, u16), Color)]) -> bool {
    points.iter().any(|x| x.0 == coords)
}

fn run(w: &mut Stdout) -> std::io::Result<()> {
    const RANDOM_FACTOR: u8 = 19;

    terminal::enable_raw_mode()?;

    loop {
        let (width, height) = terminal::size()?;

        queue!(
            w,
            style::ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(0, 0)
        )?;

        let mut already_did = vec![];
        let mut todo = vec![];

        let n_starting_points = rand::random::<usize>() % 4 + 1;

        for _ in 0..n_starting_points {
            let starting_color = Color::Rgb {
                r: rand::random::<u8>(),
                g: rand::random::<u8>(),
                b: rand::random::<u8>(),
            };

            let starting_coords = (
                rand::random::<u16>() % width,
                rand::random::<u16>() % height,
            );

            todo.push((starting_coords, starting_color));
        }

        while !todo.is_empty() {
            let index = rand::random::<usize>() % todo.len();

            let ((x, y), color) = todo.swap_remove(index);

            already_did.push(((x, y), color));

            if event::poll(Duration::from_secs(0))? {
                if let Ok(Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: KeyEventKind::Press,
                    modifiers: _,
                    state: _,
                })) = event::read()
                {
                    return Ok(());
                }
            }

            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx == 0 && dy == 0 {
                        continue;
                    }

                    if x == 0 && dx == -1 {
                        continue;
                    }
                    if y == 0 && dy == -1 {
                        continue;
                    }
                    if x >= width && dx == 1 {
                        continue;
                    }
                    if y >= height && dy == 1 {
                        continue;
                    }

                    // Safety verified above
                    let x = (x as i32 + dx) as u16;
                    let y = (y as i32 + dy) as u16;

                    let coords = (x, y);

                    if !has_coordinates(coords, &already_did) && !has_coordinates(coords, &todo) {
                        // 0-4
                        let r_change = rand::random::<u8>() % RANDOM_FACTOR;
                        let g_change = rand::random::<u8>() % RANDOM_FACTOR;
                        let b_change = rand::random::<u8>() % RANDOM_FACTOR;

                        // -2 -> 2
                        let r_change = r_change as i32 - RANDOM_FACTOR as i32 / 2;
                        let g_change = g_change as i32 - RANDOM_FACTOR as i32 / 2;
                        let b_change = b_change as i32 - RANDOM_FACTOR as i32 / 2;

                        let Color::Rgb { r, g, b } = color else {
                            panic!("Non rgb-color present");
                        };

                        let (r, g, b) = (
                            r as i32 + r_change,
                            g as i32 + g_change,
                            b as i32 + b_change,
                        );

                        let (r, g, b) = (
                            r.max(0).min(u8::MAX as i32) as u8,
                            g.max(0).min(u8::MAX as i32) as u8,
                            b.max(0).min(u8::MAX as i32) as u8,
                        );

                        todo.push((coords, Color::Rgb { r, g, b }));
                    }
                }
            }

            queue!(
                w,
                cursor::MoveTo(x, y),
                style::PrintStyledContent(StyledContent::new(
                    ContentStyle {
                        background_color: Some(color),
                        ..Default::default()
                    },
                    " ",
                ))
            )?;

            w.flush()?;

            thread::sleep(time::Duration::from_millis(1));
        }

        thread::sleep(time::Duration::from_secs(5));
    }
}

fn main() -> std::io::Result<()> {
    let mut stdout = io::stdout();

    run(&mut stdout)?;

    execute!(
        stdout,
        style::ResetColor,
        cursor::Show,
        terminal::LeaveAlternateScreen
    )?;

    terminal::disable_raw_mode()
}
