#[derive(Debug, Clone)]
pub struct Point {
    pub x: u16,
    pub y: u16,
    pub value: Option<char>,
}
impl Point {
    pub fn set_value(mut self, char: char) -> Self {
        self.value = Some(char);
        self
    }
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
    pub fn new(x: u16, y: u16, value: Option<char>) -> Point {
        Point { x, y, value }
    }
    fn _here(&self, row: u16, column: u16) -> bool {
        self.x == row && self.y == column
    }
}

#[derive(Debug, PartialEq)]
pub struct RectangleShape {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_left: Point,
    pub bottom_right: Point,
}
impl RectangleShape {
    pub fn new(start: Point, end: Point) -> RectangleShape {
        if start.y > end.y {
            if start.x > end.x {
                // _X
                // __
                return RectangleShape {
                    top_left: Point::new(end.x, start.y, None),
                    top_right: start.clone(),
                    bottom_left: end.clone(),
                    bottom_right: Point::new(start.x, end.y, None),
                };
            } else {
                // X_
                // __
                return RectangleShape {
                    top_left: start.clone(),
                    top_right: Point::new(end.x, start.y, None),
                    bottom_left: Point::new(start.x, end.y, None),
                    bottom_right: end.clone(),
                };
            }
        } else {
            if start.x > end.x {
                // __
                // _X
                return RectangleShape {
                    top_left: Point::new(start.x, end.y, None),
                    top_right: end.clone(),
                    bottom_left: Point::new(end.x, start.y, None),
                    bottom_right: start.clone(),
                };
            } else {
                // __
                // X_
                return RectangleShape {
                    top_left: Point::new(end.x, start.y, None),
                    top_right: end.clone(),
                    bottom_left: start.clone(),
                    bottom_right: Point::new(start.x, end.y, None),
                };
            }
        }
    }
}

#[derive(Debug)]
pub struct LineShape {
    pub start: Point,
    pub end: Point,
}
impl LineShape {
    pub fn new(start: Point, end: Point) -> LineShape {
        LineShape { start, end }
    }
}

#[derive(Debug)]
pub enum Shape {
    Box(RectangleShape),
    Border(RectangleShape),
    Arrow(LineShape),
    Line(LineShape),
}

#[cfg(test)]
mod tests {
    use crate::shapes::{Point, RectangleShape};
    use pretty_assertions::assert_eq;

    #[test]
    fn rectangle_top_left_to_bottom_right() {
        assert_eq!(
            RectangleShape::new(
                Point {
                    x: 2,
                    y: 5,
                    value: None
                },
                Point {
                    x: 4,
                    y: 2,
                    value: None
                }
            ),
            RectangleShape {
                top_left: Point {
                    // 1
                    x: 2,
                    y: 5,
                    value: None
                },
                top_right: Point {
                    x: 4,
                    y: 5,
                    value: None
                },
                bottom_left: Point {
                    x: 2,
                    y: 2,
                    value: None
                },
                bottom_right: Point {
                    // 2
                    x: 4,
                    y: 2,
                    value: None
                }
            }
        );
    }

    #[test]
    fn rectangle_top_right_to_bottom_left() {
        assert_eq!(
            RectangleShape::new(
                Point {
                    x: 4,
                    y: 5,
                    value: None
                },
                Point {
                    x: 2,
                    y: 2,
                    value: None
                }
            ),
            RectangleShape {
                top_left: Point {
                    x: 2,
                    y: 5,
                    value: None
                },
                top_right: Point {
                    x: 4,
                    y: 5,
                    value: None
                },
                bottom_left: Point {
                    x: 2,
                    y: 2,
                    value: None
                },
                bottom_right: Point {
                    // 2
                    x: 4,
                    y: 2,
                    value: None
                }
            }
        );
    }
}
