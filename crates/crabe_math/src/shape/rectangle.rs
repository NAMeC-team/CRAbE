use nalgebra::Point2;
use serde::Serialize;

/// A rectangle in 2D space, defined by a width, a height, and a position.
///
/// Note that the `width` and `height` fields should have the same units of
/// measurement as the coordinates of the `position` field.
#[derive(Clone, Serialize, Debug)]
pub struct Rectangle {
    /// The width of the rectangle.
    pub width: f64,
    /// The height of the rectangle.
    pub height: f64,
    /// The position of the rectangle's top-left corner.
    pub position: Point2<f64>,

}

impl Rectangle {

    pub fn new(width: f64, height: f64, position: Point2<f64>) -> Self {
        Self { width, height, position }
    }

    /// Return a boolean verifying if a point is in the rectangle
    /// 
    /// # Arguments
    /// point : position x and y on orthonormal
    /// 
    /// # Returns
    /// True if the point is in the rectangle, False otherwise
    ///  
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Rectangle;
    /// let Rec = Rectangle::new(3.,3.,Point2::new(0., 0.));
    /// assert(Rec.is_inside(Point2::new(1., 1.)));
    /// assert(Rec.is_inside(Point2::new(0., 0.)));
    /// assert(not(Rec.is_inside(Point2::new(4., 4.))));
    /// assert(not(Rec.is_inside(Point2::new(-1., 2.))));
    /// assert(not(Rec.is_inside(Point2::new(2., -1.))));
    /// ```
    pub fn is_inside(&self, point: Point2<f64>) -> bool {
        let x = point.x - self.position.x;
        let y = point.y - self.position.y;

        x >= 0.0 && x <= self.width && y >= 0.0 && y <= self.height
    }
}


