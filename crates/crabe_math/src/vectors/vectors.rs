use std::ops::Sub;

use nalgebra::{Point2, Vector2, Rotation2};

/// Returns the angle to look at a point from another point
/// 
/// # Arguments
/// p1 - The point to look from
/// p2 - The point to look at
/// 
/// # Returns
/// The angle in radians
/// 
/// # Example
/// ```
/// use crabe_math::vectors::angle_to_point;
/// use nalgebra::Point2;
/// 
/// let p1 = Point2::new(0.0, 0.0);
/// let p2 = Point2::new(1.0, 0.0);
/// let angle = angle_to_point(p1, p2);
/// assert_eq!(angle, 0.0);
/// 
/// let p3 = Point2::new(0.0, 1.0);
/// let angle2 = angle_to_point(p1, p3);
/// assert_eq!(angle2, std::f64::consts::FRAC_PI_2);
/// ```
pub fn angle_to_point(p1: Point2<f64>, p2: Point2<f64>) -> f64{
    let dir = p2.sub(p1);
    return (dir.y).atan2(dir.x);
}

/// Rotate a vector by a radian angle
/// 
/// # Arguments
/// v - The vector to rotate
/// angle - The angle in radians
/// 
/// # Returns
/// The rotated vector
/// 
/// # Example
/// ```
/// use crabe_math::vectors::rotate_vector;
/// use nalgebra::Vector2;
/// 
/// let v = Vector2::new(1.0, 0.0);
/// let angle = std::f64::consts::FRAC_PI_2;
/// let rotated_vector = rotate_vector(v, angle);
/// // assert with delta
/// assert!(rotated_vector.x - 0.0 < 0.00001);
/// assert!(rotated_vector.y - 1.0 < 0.00001);
/// ```
pub fn rotate_vector(v: Vector2<f64>, angle: f64) -> Vector2<f64> {
    let rotation = Rotation2::new(angle);
    let rotated_vector = rotation * v;
    rotated_vector
}

/// Create a vector from an a radian angle
/// 
/// # Arguments
/// angle - The angle in radians
///     
/// # Returns
/// A (1, 0) vector rotated by the angle
/// 
/// # Example
/// ```
/// use crabe_math::vectors::vector_from_angle;
/// use nalgebra::Vector2;
/// 
/// let angle = std::f64::consts::FRAC_PI_2;
/// let vector = vector_from_angle(angle);
/// // assert with delta
/// assert!(vector.x - 0.0 < 0.00001);
/// assert!(vector.y - 1.0 < 0.00001);
pub fn vector_from_angle(angle: f64) -> Vector2<f64> {
    let vector = Vector2::new(1.0, 0.0); 
    let rotation = Rotation2::new(angle);
    let rotated_vector = rotation * vector;
    rotated_vector
}