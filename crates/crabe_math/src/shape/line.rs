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

impl Line {
    pub fn new(start: Point2<f64>, end: Point2<f64>) -> Self {
        Self { start, end }
    }

    /// Return the intersection point between two lines
    /// from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection equations
    /// 
    /// # Arguments
    /// line : the line to test intersection with
    /// 
    /// # Returns
    /// The intersection point if it exists
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line1 = Line::new(Point2::new(0., 0.), Point2::new(1., 1.));
    /// let line2 = Line::new(Point2::new(0., 1.), Point2::new(1., 0.));
    /// let intersection = line1.intersection_lines(&line2);
    /// assert_eq!(intersection, Ok(Point2::new(0.5, 0.5)));
    /// let line3 = Line::new(Point2::new(2., 3.), Point2::new(4., 5.));
    /// let intersection2 = line1.intersection_lines(&line3);
    /// assert_eq!(intersection2, Err("Lines are parrallel".to_string()));
    /// let line4 = Line::new(Point2::new(-1., 0.), Point2::new(1., 0.));
    /// let line5 = Line::new(Point2::new(0., -1.), Point2::new(0., 1.));
    /// assert_eq!(line4.intersection_lines(&line5), Ok(Point2::new(0., 0.)));
    /// ```
    pub fn intersection_lines(&self, line: &Line) -> Result<Point2<f64>, String> {
        let x_nominator = (self.start.x * self.end.y - self.start.y * self.end.x)
            * (line.start.x - line.end.x)
            - (self.start.x - self.end.x) * (line.start.x * line.end.y - line.start.y * line.end.x);
        let y_nominator = (self.start.x * self.end.y - self.start.y * self.end.x)
            * (line.start.y - line.end.y)
            - (self.start.y - self.end.y) * (line.start.x * line.end.y - line.start.y * line.end.x);

        let denominator = ((self.start.x - self.end.x) * (line.start.y - line.end.y))
            - ((self.start.y - self.end.y) * (line.start.x - line.end.x));
        if denominator == 0. {
            return Err("Lines are parrallel".to_string());
        }

        let x = x_nominator / denominator;
        let y = y_nominator / denominator;
        Ok(Point2::new(x, y))
    }

    /// Return the intersection point between the current SEGMENT and a LINE
    /// (from : https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection)
    /// 
    /// # Arguments
    /// line : the line to test intersection with
    /// 
    /// # Returns
    /// The intersection point if it exists
    ///  
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line1 = Line::new(Point2::new(0., 0.), Point2::new(1., 1.));
    /// let line2 = Line::new(Point2::new(0., 1.), Point2::new(1., 0.));
    /// let intersection = line1.intersection_segment_line(&line2);
    /// assert_eq!(intersection, Ok(Point2::new(0.5, 0.5)));
    /// ```
    pub fn intersection_segment_line(&self, line: &Line) -> Result<Point2<f64>, String> {
        match self.intersection_lines(line) {
            Ok(intersection) => {
                match self.orthogonal_projection_point_on_segment(&intersection) {
                    Ok(_) => return Ok(intersection),
                    Err(_) => {}
                }
            }
            Err(_) => {}
        }
        Err("No intersection point".to_string())
    }

    /// Return the intersection point between two segments
    /// (Top of the segments are included)
    /// (from https://en.wikipedia.org/wiki/Line%E2%80%93line_intersection)
    /// 
    /// # Arguments
    /// segment : the segment to test intersection with
    /// 
    /// # Returns
    /// The intersection point if it exists
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line1 = Line::new(Point2::new(0., 0.), Point2::new(1., 1.));
    /// let line2 = Line::new(Point2::new(0., 1.), Point2::new(1., 0.));
    /// let intersection = line1.intersection_segments(&line2);
    /// assert_eq!(intersection, Ok(Point2::new(0.5, 0.5)));
    /// let line3 = Line::new(Point2::new(2., 3.), Point2::new(4., 5.));
    /// let intersection2 = line1.intersection_segments(&line3);
    /// assert_eq!(intersection2, Err("No intersection point".to_string()));
    /// let line4 = Line::new(Point2::new(-1., 0.), Point2::new(1., 0.));
    /// let line5 = Line::new(Point2::new(0., -1.), Point2::new(0., 1.));
    /// assert_eq!(line4.intersection_segments(&line5), Ok(Point2::new(0., 0.)));
    /// assert_eq!(line4.intersection_segments(&line1), Ok(Point2::new(0., 0.)));
    /// ```
    pub fn intersection_segments(&self, segment: &Line) -> Result<Point2<f64>, String> {
        let t_nominator = ((self.start.x - segment.start.x) * (segment.start.y - segment.end.y))
            - ((self.start.y - segment.start.y) * (segment.start.x - segment.end.x));
        let u_nominator = -((self.start.x - self.end.x) * (self.start.y - segment.start.y))
            - ((self.start.y - self.end.y) * (self.start.x - segment.start.x));

        let denominator = ((self.start.x - self.end.x) * (segment.start.y - segment.end.y))
            - ((self.start.y - self.end.y) * (segment.start.x - segment.end.x));
        if denominator == 0. {
            return Err("No intersection point".to_string());   
        }

        let t = t_nominator / denominator;
        let u = u_nominator / denominator;
        if t > 1. || t < 0. || u > 1. || u < 0. {
            return Err("No intersection point".to_string());
        }
        return Ok(self.point_allong_line(t));
    }

    /// Return the closest point on the line (not a segment) from a point
    /// 
    /// # Arguments
    /// point : the point to find his closest point on the line
    /// 
    /// # Returns
    /// The closest point on the line
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line = Line::new(Point2::new(0., 0.), Point2::new(0., 1.));
    /// let point = Point2::new(1., 0.5);
    /// let closest_point = line.closest_point_on_line(&point);
    /// assert_eq!(closest_point, Point2::new(0., 0.5));
    /// let point2 = Point2::new(1., 2.);
    /// let closest_point2 = line.closest_point_on_line(&point2);
    /// assert_eq!(closest_point2, Point2::new(0., 2.0));
    /// ```
    pub fn closest_point_on_line(&self, point: &Point2<f64>) -> Point2<f64> {
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;

        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, return the start point.
            return self.start;
        }
        let t = point_direction.dot(&line_direction) / line_length_squared;

        // The point is closest to a point on the segment.
        self.start + t * line_direction
    }

    /// Return the closest point on the segment (not a line) from a point
    /// 
    /// # Arguments
    /// point : the point to find his closest point on the segment
    /// 
    /// # Returns
    /// The closest point on the segment
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line = Line::new(Point2::new(0., 0.), Point2::new(0., 1.));
    /// let point = Point2::new(1., 0.5);
    /// let closest_point = line.closest_point_on_segment(&point);
    /// assert_eq!(closest_point, Point2::new(0., 0.5));
    /// let point2 = Point2::new(1., 2.);
    /// let closest_point2 = line.closest_point_on_segment(&point2);
    /// assert_eq!(closest_point2, Point2::new(0., 1.));
    /// ```
    pub fn closest_point_on_segment(&self, point: &Point2<f64>) -> Point2<f64> {
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;

        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, return the start point.
            return self.start;
        }

        let t = point_direction.dot(&line_direction) / line_length_squared;
        if t < 0.0 {
            // The point is closest to the start of the segment.
            return self.start;
        } else if t > 1.0 {
            // The point is closest to the end of the segment.
            return self.end;
        }

        // The point is closest to a point on the segment.
        self.start + t * line_direction
    }

    /// Return the closest point on the segment if it falls on him 
    /// - usefull if we need to know if the point have a projection on the segment or not
    /// - for example, lets say we want to know if the ball is in front of the penalty line, we can use this function
    /// 
    /// # Arguments
    /// point : the point to find his projection on the segment
    /// 
    /// # Returns
    /// The closest point on the segment if it falls on him
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line = Line::new(Point2::new(0., 0.), Point2::new(0., 1.));
    /// let point = Point2::new(1., 0.5);
    /// let closest_point = line.orthogonal_projection_point_on_segment(&point);
    /// assert_eq!(closest_point, Ok(Point2::new(0., 0.5)));
    /// let point2 = Point2::new(1., 2.);
    /// let closest_point2 = line.orthogonal_projection_point_on_segment(&point2);
    /// assert_eq!(closest_point2, Err("The point don't fall on the segment".to_string()));
    /// ```
    pub fn orthogonal_projection_point_on_segment(
        &self,
        point: &Point2<f64>,
    ) -> Result<Point2<f64>, String> {
        let line_direction = self.end - self.start;
        let point_direction = *point - self.start;

        let line_length_squared = line_direction.norm_squared();
        if line_length_squared == 0.0 {
            // The line segment has zero length, should we return the segment point ?
            return Err("The line segment has zero length".to_string());
        }

        let t = point_direction.dot(&line_direction) / line_length_squared;
        if t < 0. || t > 1. {
            return Err("The point don't fall on the segment".to_string());
        } // The point don't fall on the segment.

        // The point is closest to a point on the segment.
        Ok(self.start + t * line_direction)
    }

    /// Returns a point along the line at a specified distance from the start point.
    /// based on a given fraction of the total distance between the two points.
    /// - for example with 0. it will return the starting point
    /// - and with 0.5, it will return the center of the line
    /// 
    /// # Arguments
    /// x : the fraction of the total distance between the two points (between 0. and 1.)
    /// 
    /// # Returns
    /// The point at the specified distance from the start point
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line = Line::new(Point2::new(1., 0.2), Point2::new(1., 1.));
    /// let point = line.point_allong_line(0.5);
    /// let delta = point - Point2::new(1., 0.6);
    /// assert!(delta.norm() < 0.000001);
    /// ```
    pub fn point_allong_line(&self, x: f64) -> Point2<f64> {
        let p_x = self.start.x + x * (self.end.x - self.start.x);
        let p_y = self.start.y + x * (self.end.y - self.start.y);
        return Point2::new(p_x, p_y);
    }

    /// Return the center point of the segment
    /// 
    /// # Returns
    /// The center point of the segment
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line = Line::new(Point2::new(1., 0.2), Point2::new(1., 1.));
    /// let center = line.center();
    /// let delta = center - Point2::new(1., 0.6);
    /// assert!(delta.norm() < 0.000001);
    /// ```
    pub fn center(&self) -> Point2<f64> {
        return self.point_allong_line(0.5);
    }
 
    /// Return the length of the line
    /// 
    /// # Returns
    /// The length of the line
    /// 
    /// # Example
    /// ```
    /// use nalgebra::Point2;
    /// use crabe_math::shape::Line;
    /// let line = Line::new(Point2::new(1., 0.2), Point2::new(1., 1.));
    /// let length = line.norm();
    /// assert_eq!(length, 0.8);
    /// ```
    pub fn norm(&self) -> f64 {
        let vec = self.end - self.start;
        vec.norm()
    }
}
