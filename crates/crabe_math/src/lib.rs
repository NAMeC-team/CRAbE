//! A collection of math-related modules used in various parts of the CRAbE system.
//!
//! This crate includes the following modules:
//!
//! * `shape`: contains geometric primitives, such as `Line` and `Circle`, used to represent and manipulate shapes.

/// The `shape` module contains geometric primitives, such as `Line` and `Circle`,
/// which are used in various parts of the system to represent and manipulate shapes.
pub mod shape;





pub mod vectors;

#[cfg(test)]
mod tests {
    use nalgebra::Point2;

    use crate::shape::Line;

    #[test]
    fn test_line_intersection() {
        let line1 = Line::new(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0));
        let line2 = Line::new(Point2::new(1.0, 0.0), Point2::new(1.0, 3.0));
        let line3 = Line::new(Point2::new(0.0, 1.0), Point2::new(3.0, 1.0));

        assert_eq!(line1.intersection(&line2),Some(Point2::new(1.0, 1.0)));
        assert_eq!(line1.intersection(&line3),Some(Point2::new(1.0, 1.0)));
        assert_eq!(line2.intersection(&line3),Some(Point2::new(1.0, 1.0)));

        let line_target = Line::new(Point2::new(0.0, -1.0), Point2::new(0.0, 1.0));
        let line_htop = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 3.0));
        //let line_top = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 2.0));
        let line_mid = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 0.0));
        //let line_bot = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, -2.0));
        let line_hbot = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, -3.0));

        assert_eq!(line_target.intersection(&line_htop),None);
        //assert_eq!(line_target.intersection(&line_top),Some(Point2::new(0.0, 1.0)));
        assert_eq!(line_target.intersection(&line_mid),Some(Point2::new(0.0, 0.0)));
        //assert_eq!(line_target.intersection(&line_bot),Some(Point2::new(0.0, -1.0)));
        assert_eq!(line_target.intersection(&line_hbot),None);
    }

    #[test]
    fn test_line_intersect() {
        let line1 = Line::new(Point2::new(0.0, 0.0), Point2::new(2.0, 2.0));
        let line2 = Line::new(Point2::new(1.0, 0.0), Point2::new(1.0, 3.0));
        let line3 = Line::new(Point2::new(0.0, 1.0), Point2::new(3.0, 1.0));

        assert_eq!(line1.intersect(&line2),true);
        assert_eq!(line1.intersect(&line3),true);
        assert_eq!(line2.intersect(&line3),true);

        let line_target = Line::new(Point2::new(0.0, -1.0), Point2::new(0.0, 1.0));
        let line_htop = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 3.0));
        //let line_top = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 2.0));
        let line_mid = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, 0.0));
        //let line_bot = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, -2.0));
        let line_hbot = Line::new(Point2::new(1.0, 0.0), Point2::new(-1.0, -3.0));

        assert_eq!(line_target.intersect(&line_htop),false);
        //assert_eq!(line_target.intersect(&line_top),true);
        assert_eq!(line_target.intersect(&line_mid),true);
        //assert_eq!(line_target.intersect(&line_bot),true);
        assert_eq!(line_target.intersect(&line_hbot),false);
    }
}
