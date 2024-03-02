use anyhow::Result;
use std::io::{stdout, Write};

use crossterm::{cursor, ExecutableCommand, QueueableCommand};

use crate::{
    shapes::{Point, RectangleShape, Shape},
    State,
};

///┌───────────┐
///│           │
///│           │
///│           │
///│           │
///│           │
///└───────────┘
fn calculte_border(rect: &RectangleShape) -> Vec<Point> {
    let mut out = vec![];
    // top/bottom line
    for y in rect.top_left.y..rect.top_right.y {
        out.push(Point::new(rect.top_left.x, y, Some('█')));
        out.push(Point::new(rect.bottom_left.x, y, Some('█')));
    }

    for x in rect.top_right.x..rect.top_left.x {
        out.push(Point::new(x, rect.top_left.y, Some('█')));
        out.push(Point::new(x, rect.bottom_left.y, Some('█')));
    }

    out.dedup();
    out
}

///──────►
fn calculte_arrow(start: &Point, end: &Point) -> Vec<Point> {
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
fn calculte_box(rect: &RectangleShape) -> Vec<Point> {
    let mut out = vec![];
    for x in rect.top_right.x..rect.bottom_left.x {
        for y in rect.top_right.y..rect.bottom_left.y {
            out.push(Point::new(x, y, Some('█')));
        }
    }

    out
}

fn calculte_points(start: &Point, end: &Point) -> Vec<Point> {
    vec![start.clone().set_value('X'), end.clone().set_value('X')]
}

///┌─────────┐       ┌───────────┐
///│         │       │           │
///│         │       │           │
///│         ├──────►│           │
///│         │       │           │
///│         │       │           │
///└─────────┘       └───────────┘
pub fn render_canvas(state: &State, _theight: &u16, _twith: &u16) -> Result<()> {
    for shape in &state.shapes {
        let points = match shape {
            Shape::Box(rec) => calculte_box(rec),
            Shape::Border(rec) => calculte_border(rec),
            //Shape::Arrow(line) => calculte_arrow(&line.start, &line.end),
            //Shape::Box(rec) => calculte_points(&rec.top_right, &rec.bottom_left),
            //Shape::Border(rec) => calculte_points(&rec.top_right, &rec.bottom_left),
            Shape::Arrow(line) => calculte_points(&line.start, &line.end),
            Shape::Line(_line) => vec![],
        };
        render_points(points)?;
    }

    Ok(())
}

fn render_points(points: Vec<Point>) -> Result<()> {
    let mut stdout = stdout();

    for point in points {
        stdout.execute(cursor::MoveTo(point.x, point.y))?;
        let Some(char) = point.value else {
            continue;
        };
        stdout.write(char.to_string().as_bytes())?;
    }

    Ok(())
}
