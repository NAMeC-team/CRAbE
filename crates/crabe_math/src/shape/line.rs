use nalgebra::Point2;
use serde::Serialize;

/// A line segment in 2D space, defined by two points.
///
/// Note that the `start` and `end` fields should have the same units of
/// measurement.
#[derive(Clone, Serialize, Debug)]
pub struct Line {
    /// The starting point of the line segment.
    pub start: Point2<f64>,
    /// The ending point of the line segment.
    pub end: Point2<f64>,
}

impl Line{
    pub fn new(start: Point2<f64>, end: Point2<f64>) -> Self {
        Self { 
            start,
            end
        }
    }
    pub fn intersect(&mut self, line: Line) -> bool{
        let a = self.start;
        let b = self.end;
        let c = line.start;
        let d = line.end;
    
        ccw(a, c, d) != ccw(b, c, d) && ccw(a, b, c) != ccw(a, b, d)
    }
    pub fn intersection(&mut self, line: Line) -> Option<Point2<f64>>{
        let xdiff = (self.start.x - self.end.x, line.start.x - line.end.x);
        let ydiff = (self.start.y - self.end.y, line.start.y - line.end.y);
    
        fn det(a: (f64, f64), b: (f64, f64)) -> f64 {
            a.0 * b.1 - a.1 * b.0
        }
    
        let div = det(xdiff, ydiff);
        if div.abs() < 1e-6 {
            println!("Lines do not intersect");
            return None;
        }
    
        let d = (det((self.start.x, self.start.y), (self.end.x, self.end.y)), det((line.start.x, line.start.y), (line.end.x, line.end.y)));
        let x = det(d, xdiff) / div;
        let y = det(d, ydiff) / div;
        Some(Point2::new(x, y))}
}

fn ccw(a: Point2<f64>, b: Point2<f64>, c: Point2<f64>) -> bool {
    (c.y - a.y) * (b.x - a.x) >= (b.y - a.y) * (c.x - a.x)
}
