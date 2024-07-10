use nalgebra::Point2;
use serde::Serialize;

/// Represents a circle in 2D space defined by its center point and radius.
///
/// Note that the `center` and `radius` fields should have the same units of
/// measurement.
#[derive(Serialize, Clone, Debug)]
pub struct Circle {
    /// The center point of the circle.
    pub center: Point2<f64>,
    /// The radius of the circle.
    pub radius: f64,
}

impl Circle {
    /// Create a new circle with the given center and radius.
    pub fn new(center: Point2<f64>, radius: f64) -> Self {
        Self { center, radius }
    }

    /// Return a boolean verifying if a point is in the circle
    ///     
    /// # Arguments
    /// point : position x and y on orthonormal
    /// 
    /// # Returns
    /// True if the point is in the circle, False otherwise
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Circle;
    /// let circle = Circle::new(Point2::new(0., 0.), 3.);
    /// assert!(circle.is_inside(Point2::new(1., 1.)));
    /// assert!(circle.is_inside(Point2::new(0., 0.)));
    /// assert!(!circle.is_inside(Point2::new(4., 4.)));
    /// assert!(!circle.is_inside(Point2::new(-1., 3.)));
    /// assert!(!circle.is_inside(Point2::new(3., -1.)));
    /// ```
    pub fn is_inside(&self, point: Point2<f64>) -> bool {
        (point - self.center).norm() <= self.radius
    }
}