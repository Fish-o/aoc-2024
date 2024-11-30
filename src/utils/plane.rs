use itertools::{sorted, Itertools};
use num::{FromPrimitive, Rational64};
use std::cmp::{max, min};

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Hash)]
pub struct Point {
    pub x: Rational64,
    pub y: Rational64,
}
impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
impl Point {
    pub fn from_xy(x: i64, y: i64) -> Point {
        Point {
            x: Rational64::from_i64(x).unwrap(),
            y: Rational64::from_i64(y).unwrap(),
        }
    }
}
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Line {
    pub a: Point,
    pub b: Point,
}
impl Line {
    pub fn from(p1: Point, p2: Point) -> Line {
        Line { a: p1, b: p2 }
    }
}
impl Line {
    pub fn to_points(&self) -> Vec<Point> {
        if self.is_vertical() {
            let y1 = &self.a.y;
            let y2 = &self.b.y;
            let (sy, ey) = (min(y1, y2).to_integer(), max(y1, y2).to_integer());
            (sy..=ey)
                .map(|y| Point {
                    y: y.into(),
                    x: self.a.x.clone(),
                })
                .collect_vec()
        } else {
            let x1 = &self.a.x;
            let x2 = &self.b.x;
            let (sx, ex) = (min(x1, x2).to_integer(), max(x1, x2).to_integer());
            let (a, b) = self.get_ab();
            (sx..=ex)
                .map(|x| Point::from_xy(x, (a * x + b).to_integer()))
                .collect_vec()
        }
    }
    pub fn is_vertical(&self) -> bool {
        self.a.x == self.b.x
    }
    pub fn is_horizontal(&self) -> bool {
        self.a.y == self.b.y
    }
    fn get_ab(&self) -> (Rational64, Rational64) {
        let x1 = &self.a.x;
        let y1 = &self.a.y;
        let x2 = &self.b.x;
        let y2 = &self.b.y;
        let a1 = (y1 - y2) / (x1 - x2);
        let b1 = y1 - &a1 * x1;
        (a1, b1)
    }
    pub fn intersection(&self, other: &Line) -> Option<Point> {
        let x1 = &self.a.x;
        let x2 = &self.b.x;
        let x3 = &other.a.x;
        let x4 = &other.b.x;

        if max(x1, x2) < min(x3, x4) {
            return None;
        } else if self.is_vertical() && other.is_vertical() {
            return None;
        } else if self.is_vertical() || other.is_vertical() {
            let (ver, other) = if self.is_vertical() {
                (self, other)
            } else {
                (other, self)
            };
            let x = &ver.a.x;
            let (sx, ex) = (min(&other.a.x, &other.b.x), max(&other.a.x, &other.b.x));
            if !(sx <= x && x <= ex) {
                return None;
            }
            let (sy, ey) = (min(&ver.a.y, &ver.b.y), max(&ver.a.y, &ver.b.y));
            let (a, b) = other.get_ab();
            let y = &a * x + &b;
            return if sy <= &y && &y <= ey {
                Some(Point {
                    x: x.clone(),
                    y: y.clone(),
                })
            } else {
                None
            };
        }
        // f1(x) = A1*x + b1 = y
        // f2(x) = A2*x + b2 = y
        let (a1, b1) = self.get_ab();
        let (a2, b2) = other.get_ab();
        if &a1 == &a2 {
            // They are parallel
            return None;
        }

        let xa = (&b2 - &b1) / (&a1 - &a2);
        if (&xa < max(min(x1, x2), min(x3, x4))) || (&xa > min(max(x1, x2), max(x3, x4))) {
            return None;
            //  # intersection is out of bound
        }
        Some(Point {
            x: (xa).clone(),
            y: (a1 * xa + b1).clone(),
        })
    }
    pub fn overlap(&self, other: &Line) -> Option<Line> {
        if self.is_vertical() && other.is_vertical() {
            let x = &self.a.x;
            if x != &other.a.x {
                return None;
            }
            let y1 = &self.a.y;
            let y2 = &self.b.y;
            let y3 = &other.a.y;
            let y4 = &other.b.y;
            let (s1, e1) = (min(y1, y2), max(y1, y2));
            let (s2, e2) = (min(y3, y4), max(y3, y4));
            if e1 < s2 || e2 < s1 {
                return None;
            }
            let so = if s1 <= s2 && s2 <= e1 {
                s2
            } else if s2 <= s1 && s1 <= e2 {
                s1
            } else {
                panic!("How did we get here s???")
            };
            let eo = if s1 <= e2 && e2 <= e1 {
                e2
            } else if s2 <= e1 && e1 <= e2 {
                e1
            } else {
                panic!("How did we get here e???")
            };
            return Some(Line::from(
                Point {
                    x: x.clone(),
                    y: so.clone(),
                },
                Point {
                    x: x.clone(),
                    y: eo.clone(),
                },
            ));
        } else if self.is_vertical() || other.is_vertical() {
            return None;
        }
        let (a1, b1) = self.get_ab();
        let (a2, b2) = other.get_ab();
        if a1 != a2 || b1 != b2 {
            return None;
        }

        let x1 = &self.a.x;
        let x2 = &self.b.x;
        let x3 = &other.a.x;
        let x4 = &other.b.x;
        let (s1, e1) = (min(x1, x2), max(x1, x2));
        let (s2, e2) = (min(x3, x4), max(x3, x4));
        if e1 < s2 || e2 < s1 {
            return None;
        }
        let so = if s1 <= s2 && s2 <= e1 {
            s2
        } else if s2 <= s1 && s1 <= e2 {
            s1
        } else {
            panic!("How did we get here s???")
        };
        let eo = if s1 <= e2 && e2 <= e1 {
            e2
        } else if s2 <= e1 && e1 <= e2 {
            e1
        } else {
            panic!("How did we get here e???")
        };
        Some(Line::from(
            Point {
                x: so.clone(),
                y: so * &a1 + &b1,
            },
            Point {
                x: eo.clone(),
                y: eo * &a1 + &b1,
            },
        ))
    }

    pub fn contains_point(&self, point: &Point) -> bool {
        if self.is_vertical() {
            let y1 = &self.a.y;
            let y2 = &self.b.y;
            let (y1, y2) = (min(y1, y2), max(y1, y2));
            return point.x == self.a.x && y1 <= &point.y && &point.y <= y2;
        }

        let x1 = &self.a.x;
        let x2 = &self.b.x;
        let (s1, e1) = (min(x1, x2), max(x1, x2));
        if !(s1 <= &point.x && &point.x <= e1) {
            return false;
        }
        let (a, b) = self.get_ab();
        let y = a * &point.x + b;
        y == point.y
    }
}

impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.a, self.b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(2, 0), Point::from_xy(0, 2));
        assert_eq!(l1.intersection(&l2), Some(Point::from_xy(1, 1)));
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(2, 4), Point::from_xy(0, 6));
        assert_eq!(l1.intersection(&l2), None);
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(2, 2), Point::from_xy(4, 4));
        assert_eq!(l1.intersection(&l2), None);
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(1, 1), Point::from_xy(3, 3));
        assert_eq!(l1.intersection(&l2), None);
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(1, 1), Point::from_xy(3, 3));
        assert_eq!(l1.intersection(&l2), None);
        let l1 = Line::from(Point::from_xy(-1, -1), Point::from_xy(2, -1));
        let l2 = Line::from(Point::from_xy(-1, -2), Point::from_xy(1, 0));
        assert_eq!(l1.intersection(&l2), Some(Point::from_xy(0, -1)));
        let l1 = Line::from(Point::from_xy(-1, 0), Point::from_xy(0, 0));
        let l2 = Line::from(Point::from_xy(0, 0), Point::from_xy(0, 1));
        assert_eq!(l1.intersection(&l2), Some(Point::from_xy(0, 0)));
    }
    #[test]
    fn test_overlap() {
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        assert_eq!(l1.overlap(&l2), Some(l1.clone()));
        // println!("{:?}", l1.overlap(&l2));
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(1, 1), Point::from_xy(2, 2));
        assert_eq!(l1.overlap(&l2), Some(l2.clone()));
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(-1, -1), Point::from_xy(1, 1));
        assert_eq!(
            l1.overlap(&l2),
            Some(Line::from(Point::from_xy(0, 0), Point::from_xy(1, 1)))
        );
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(1, 0), Point::from_xy(3, 2));
        assert_eq!(l1.overlap(&l2), None);
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        let l2 = Line::from(Point::from_xy(2, 2), Point::from_xy(4, 4));
        assert_eq!(
            l1.overlap(&l2),
            Some(Line::from(Point::from_xy(2, 2), Point::from_xy(2, 2)))
        );
    }
    #[test]
    pub fn test_to_points() {
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(2, 2));
        assert_eq!(
            l1.to_points(),
            vec![
                Point::from_xy(0, 0),
                Point::from_xy(1, 1),
                Point::from_xy(2, 2)
            ]
        );
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(0, 2));
        assert_eq!(
            l1.to_points(),
            vec![
                Point::from_xy(0, 0),
                Point::from_xy(0, 1),
                Point::from_xy(0, 2)
            ]
        );
        let l1 = Line::from(Point::from_xy(2, 0), Point::from_xy(0, 0));
        assert_eq!(
            l1.to_points(),
            vec![
                Point::from_xy(0, 0),
                Point::from_xy(1, 0),
                Point::from_xy(2, 0)
            ]
        );
        let l1 = Line::from(Point::from_xy(0, 0), Point::from_xy(0, 0));
        assert_eq!(l1.to_points(), vec![Point::from_xy(0, 0),]);
    }
}
