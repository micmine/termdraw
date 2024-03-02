pub mod shapes;
pub mod render;

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
    terminal::{
        self, disable_raw_mode, enable_raw_mode, EnterAlternateScreen,
        LeaveAlternateScreen, ClearType,
    },
    ExecutableCommand, QueueableCommand,
};
use render::render_canvas;
use shapes::{Shape, RectangleShape, LineShape, Point};
use tracing::{info, Level};

#[derive(Debug)]
pub struct State {
    shapes: Vec<Shape>,
}
//#[derive(Debug)]
//struct ViewVisor<'a> {
//center: Option<&'a Point>,
//top: Option<&'a Point>,
//bottom: Option<&'a Point>,
//right: Option<&'a Point>,
//left: Option<&'a Point>,
//}

#[derive(Debug, PartialEq, Eq)]
enum DrawMode {
    Box,
    Border,
    Line,
    Arrow,
}

impl State {
    //fn get_view_visor(&self, x: u16, y: u16) -> ViewVisor {
    //let mut out = ViewVisor {
    //center: None,
    //top: None,
    //bottom: None,
    //left: None,
    //right: None,
    //};
    //for point in &self.points {
    //// center
    //if point.here(x, y) {
    //out.center = Some(point);
    //}
    //// top
    //if x > 0 {
    //if point.here(x - 1, y) {
    //out.top = Some(point);
    //}
    //}
    //// bottom
    //if point.here(x + 1, y) {
    //out.bottom = Some(point);
    //}
    //// left
    //if y > 0 {
    //if point.here(x, y - 1) {
    //out.left = Some(point);
    //}
    //}
    //// right
    //if point.here(x, y + 1) {
    //out.right = Some(point);
    //}
    //}

    //out
    //}
    fn insert(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    fn new() -> State {
        State { shapes: vec![] }
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
    let mut drag_start: Option<Point> = None;
    let mut mode = DrawMode::Box;
    'ui: loop {
        while poll(Duration::ZERO)? {
            match read()? {
                Event::Key(m) => match m.code {
                    KeyCode::Char('b') => mode = DrawMode::Border,
                    KeyCode::Char('s') => mode = DrawMode::Box,
                    KeyCode::Char('l') => mode = DrawMode::Line,
                    KeyCode::Char('a') => mode = DrawMode::Arrow,
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break 'ui;
                    }
                    _ => (),
                },
                Event::Mouse(m) => match m.kind {
                    MouseEventKind::Down(MouseButton::Left) => {
                        drag_start = Some(Point::new(m.column, m.row, None));
                    }
                    MouseEventKind::Up(MouseButton::Left) => {
                        if let Some(ref start) = drag_start {
                            let end = Point::new(m.column, m.row, None);
                            //let mut file = File::open("/tmp/termdraw").unwrap();
                            //file.write_all().unwrap();
                            //file.flush().unwrap();
                            //stdout.execute(cursor::MoveTo(0,0))?;
                            //stdout.write_all(format!("{:?}, {:?}", &start, &end).as_bytes())?;
                            let shape = match mode {
                                DrawMode::Box => {
                                    Shape::Box(RectangleShape::new(start.clone(), end))
                                }
                                DrawMode::Arrow => Shape::Arrow(LineShape::new(start.clone(), end)),
                                DrawMode::Line => Shape::Line(LineShape::new(start.clone(), end)),
                                DrawMode::Border => {
                                    Shape::Border(RectangleShape::new(start.clone(), end))
                                }
                            };
                            state.insert(shape);
                        }
                    }
                    MouseEventKind::Drag(MouseButton::Left) => {
                        //if mode == DrawMode::Line {
                        //state.insert(Point::new(m.row, m.column, Some('█')));
                        //}
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

        stdout.queue(cursor::MoveTo(0, 0))?;
        stdout.execute(terminal::Clear(ClearType::All))?;
        render_canvas(&state, &theight, &twith)?;
        stdout.flush()?;
        //stdout.write(canvas.as_bytes())?;
        thread::sleep(Duration::from_millis(10));
    }
    teardown_ui()?;
    Ok(())
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
