use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

use anyhow::Result;
use crossterm::{
    cursor::{self, Hide, Show},
    event::{poll, Event, MouseButton, MouseEventKind},
    event::{read, DisableMouseCapture, EnableMouseCapture, KeyCode},
    terminal::{self, disable_raw_mode, enable_raw_mode, LeaveAlternateScreen, EnterAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use tracing::{info, Level};

#[derive(Debug)]
pub struct State {
    points: Vec<Point>,
}
#[derive(Debug)]
struct ViewVisor<'a> {
    center: Option<&'a Point>,
    top: Option<&'a Point>,
    bottom: Option<&'a Point>,
    right: Option<&'a Point>,
    left: Option<&'a Point>,
}

#[derive(Debug, PartialEq, Eq)]
enum DrawMode {
    Box,
    Line,
    Border,
    Arrow,
}

impl State {
    fn get_view_visor(&self, x: u16, y: u16) -> ViewVisor {
        let mut out = ViewVisor {
            center: None,
            top: None,
            bottom: None,
            left: None,
            right: None,
        };
        for point in &self.points {
            // center
            if point.here(x, y) {
                out.center = Some(point);
            }
            // top
            if x > 0 {
                if point.here(x - 1, y) {
                    out.top = Some(point);
                }
            }
            // bottom
            if point.here(x + 1, y) {
                out.bottom = Some(point);
            }
            // left
            if y > 0 {
                if point.here(x, y - 1) {
                    out.left = Some(point);
                }
            }
            // right
            if point.here(x, y + 1) {
                out.right = Some(point);
            }
        }

        out
    }
    fn insert(&mut self, point: Point) {
        if !self.points.contains(&point) {
            self.points.push(point);
        }
    }
    fn insert_meany(&mut self, points: Vec<Point>) {
        for point in points {
            self.insert(point);
        }
    }

    fn new() -> State {
        State { points: vec![] }
    }

    fn zoom(&self, theight: u16, twith: u16) -> Vec<&Point> {
        self.points
            .iter()
            .filter(|p| p.x < theight && p.y < twith)
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Point {
    x: u16,
    y: u16,
    value: Option<char>,
}
impl PartialEq for Point {
    fn ne(&self, other: &Self) -> bool {
        !self.x.eq(&other.x) && !self.y.eq(&other.y)
    }

    fn eq(&self, other: &Self) -> bool {
        self.x.eq(&other.x) && self.y.eq(&other.y)
    }
}
impl Point {
    fn new(row: u16, column: u16, value: Option<char>) -> Point {
        Point {
            x: row,
            y: column,
            value,
        }
    }
    fn here(&self, row: u16, column: u16) -> bool {
        self.x == row && self.y == column
    }
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::hourly(".", "termdraw.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(non_blocking)
        .init();
    ui().unwrap();
}

fn ui() -> Result<()> {
    let (mut theight, mut twith) = terminal::size()?;
    let mut state = State::new();
    setup_ui()?;
    let mut stdout = stdout();
    let mut drap_start: Option<Point> = None;
    let mut mode = DrawMode::Line;
    'ui: loop {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(m) => {
                    match m.code {
                        KeyCode::Char('b') => mode = DrawMode::Border,
                        KeyCode::Char('s') => mode = DrawMode::Box,
                        KeyCode::Char('l') => mode = DrawMode::Line,
                        KeyCode::Char('a') => mode = DrawMode::Arrow,
                        KeyCode::Esc => {
                            break 'ui;
                        }
                        _ => (),
                    }
                }
                Event::Mouse(m) => match m.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        drap_start = Some(Point::new(m.row, m.column, None));
                    }
                    MouseEventKind::Up(MouseButton::Left) => {
                        if let Some(ref start) = drap_start {
                            let end = Point::new(m.row, m.column, None);
                            let new_elements = match mode {
                                DrawMode::Box => calculte_box(start, end),
                                DrawMode::Line => vec![],
                                DrawMode::Arrow => calculte_arrow(start, end),
                                DrawMode::Border => calculte_border(start, end),
                            };
                            state.insert_meany(new_elements);
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        if mode == DrawMode::Line {
                            state.insert(Point::new(m.row, m.column, Some('█')));
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Right) => {}
                    _ => (),
                },
                Event::Resize(nw, nh) => {
                    theight = nh;
                    twith = nw;
                }
                _ => (),
            }
        }

        //stdout.execute(terminal::Clear(ClearType::All))?;
        if let Ok(canvas) = render_canvas(&state, &theight, &twith) {
            stdout.queue(cursor::MoveTo(0, 0))?;
            stdout.write(canvas.as_bytes())?;
        }
        stdout.flush()?;
        thread::sleep(Duration::from_millis(10));
    }
    teardown_ui()?;
    Ok(())
}

///┌───────────┐
///│           │
///│           │
///│           │
///│           │
///│           │
///└───────────┘
fn calculte_border(start: &Point, end: Point) -> Vec<Point> {
    let mut out = vec![];
    // top/bottom line
    for y in start.y..end.y {
        out.push(Point::new(start.x, y, Some('█')));
        out.push(Point::new(end.x, y, Some('█')));
    }

    for x in start.x..end.x {
        out.push(Point::new(x, start.y, Some('█')));
        out.push(Point::new(x, end.y, Some('█')));
    }

    out.dedup();
    out
}

///──────►
fn calculte_arrow(start: &Point, end: Point) -> Vec<Point> {
    let mut out = vec![];
    // top/bottom line
    for y in start.y..end.y {
        out.push(Point::new(start.x, y, Some('─')));
    }
    out.push(Point::new(start.x, end.y, Some('►')));

    out
}
///█████████████
///█████████████
///█████████████
///█████████████
///█████████████
fn calculte_box(start: &Point, end: Point) -> Vec<Point> {
    let mut out = vec![];
    for x in start.x..end.x {
        for y in start.y..end.y {
            out.push(Point::new(x, y, Some('█')));
        }
    }

    out
}

///┌─────────┐       ┌───────────┐
///│         │       │           │
///│         │       │           │
///│         ├──────►│           │
///│         │       │           │
///│         │       │           │
///└─────────┘       └───────────┘
fn render_canvas(state: &State, theight: &u16, twith: &u16) -> Result<String> {
    //let points = state.zoom(*theight, *twith);

    let mut canvas = String::with_capacity((theight * twith).into());

    for x in 0..*theight {
        for y in 0..*twith {
            let visor = state.get_view_visor(x, y);
            if let Some(center) = visor.center {
                canvas.push(center.value.unwrap_or('U'));
            } else {
                canvas.push(' ');
            }
        }
    }

    Ok(canvas)
}

fn setup_ui() -> Result<()> {
    info!("Starting");
    enable_raw_mode()?;
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(EnableMouseCapture)?;
    stdout.execute(Hide)?;

    Ok(())
}

fn teardown_ui() -> Result<()> {
    let mut stdout = stdout();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(DisableMouseCapture)?;
    stdout.execute(Show)?;
    disable_raw_mode()?;
    Ok(())
}
