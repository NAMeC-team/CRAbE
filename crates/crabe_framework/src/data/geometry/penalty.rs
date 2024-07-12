use nalgebra::Point2;
use serde::Serialize;
use crabe_math::shape::Line;

use crate::data::world::World;

/// Represents a penalty area on a soccer field. (all distances are in meters)
#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Penalty {
    /// The width of the penalty area 
    pub width: f64,
    /// The depth of the penalty area 
    pub depth: f64,
    /// The front line of the penalty area
    pub front_line: Line,
    /// The back line of the penalty area
    pub back_line: Line,
    /// The left line of the penalty area (looking from the center of the field)
    pub left_line: Line,
    /// The right line of the penalty area (looking from the center of the field)
    pub right_line: Line,
}

impl Penalty {
    /// Return a point on the penalty line from a number between 0 and 1
    /// 0 is the left corner of the penalty zone, 1 is the right corner
    /// 0.5 is the center of the penalty zone
    /// 
    /// # Arguments
    /// - `world`: The current world state.
    /// - `x`: The position from 0 to 1 along the penalty zone.
    /// 
    /// # Returns
    /// The point on the penalty line.
    pub fn on_penalty_line(
        &self,
        x: f64
    ) -> Point2<f64> { 
        let penalty_line_length = self.depth * 2. + self.width;
        let dist_along_penalty_line = penalty_line_length * x;
        if dist_along_penalty_line < self.depth{
            let n_ratio = dist_along_penalty_line / self.depth;
            return self.left_line.point_allong_line(n_ratio);
        }else if dist_along_penalty_line < self.depth + self.width{
            let n_ratio = (dist_along_penalty_line - self.depth)/self.width;
            return self.front_line.point_allong_line(n_ratio);
        }else{
            let n_ratio = 1. - (dist_along_penalty_line - (self.depth + self.width))/self.depth;
            return self.right_line.point_allong_line(n_ratio);
        }
    }    

    fn intersection(&self, line: Line, segment_intersection: bool) -> (Option<Point2<f64>>, Option<f64>) {
        let penalty_line_length = self.depth * 2. + self.width;
        let intersection_front_line = if segment_intersection {self.front_line.intersection_segments(&line)} else {self.front_line.intersection_segment_line(&line)};
        if let Ok(intersection) = intersection_front_line {
            let front_line_point_as_ratio = ((intersection.y - self.front_line.start.y).abs() + self.depth)/penalty_line_length;
            return (Some(intersection), Some(front_line_point_as_ratio));
        }
        let intersection_left_line = if segment_intersection {line.intersection_segments(&self.left_line)} else {self.left_line.intersection_segment_line(&line)};
        let intersection_right_line = if segment_intersection {line.intersection_segments(&self.right_line)} else {self.right_line.intersection_segment_line(&line)};
        if intersection_left_line.is_ok() && intersection_right_line.is_ok() {
            //check closest point between the two intersections
            let left = intersection_left_line.unwrap();
            let right = intersection_right_line.unwrap();
            let left_dist = (left - line.start).norm();
            let right_dist = (right - line.start).norm();
            if left_dist < right_dist {
                let left_line_point_as_ratio = (left.x - self.left_line.start.x).abs()/penalty_line_length;
                return (Some(left), Some(left_line_point_as_ratio));
            }
            let right_line_point_as_ratio = ((right.x - self.right_line.start.x).abs() + self.depth + self.width)/penalty_line_length;
            return (Some(right), Some(right_line_point_as_ratio));
        }
        if let Ok(intersection) = intersection_left_line {
            let left_line_point_as_ratio = (intersection.x - self.left_line.start.x).abs()/penalty_line_length;
            return (Some(intersection), Some(left_line_point_as_ratio));
        }else if let Ok(intersection) = intersection_right_line {
            let right_line_point_as_ratio = ((intersection.x - self.right_line.end.x).abs()+ self.depth + self.width)/penalty_line_length;
            return (Some(intersection), Some(right_line_point_as_ratio));
        }
        (None,None)
    }

    /// Return a number between 0 and 1 representing the position of a point on the penalty line intersecting with a line
    /// 0 is the left corner of the penalty zone, 1 is the right corner
    /// 0.5 is the center of the penalty zone
    /// take the start point of the line as reference for which side of the penalty zone the point is (in case intersection with multiple line)
    /// 
    /// # Arguments
    /// - `line`: The line to check intersection with.
    /// - `strict_segment_intersection`: true : check intersection with segment line; false : check intersection with infinite line
    /// 
    /// # Returns
    /// A number between 0 and 1 representing the position of the intersection point on the penalty line.
    pub fn intersection_line_as_ratio(
        &self,
        line: Line
    ) ->  Option<f64>{
        let (_, ratio) = self.intersection(line, false);
        return ratio;
    }

    /// Return a number between 0 and 1 representing the position of a point on the penalty line intersecting with a segment
    /// 0 is the left corner of the penalty zone, 1 is the right corner
    /// 0.5 is the center of the penalty zone
    /// take the start point of the segment as reference for which side of the penalty zone the point is (in case intersection with multiple line)
    /// 
    /// # Arguments
    /// - `segment`: The segment to check intersection with.
    /// 
    /// # Returns
    /// A number between 0 and 1 representing the position of the intersection point on the penalty line.
    pub fn intersection_segment_as_ratio(
        &self,
        segment: Line
    ) ->  Option<f64>{
        let (_, ratio) = self.intersection(segment, true);
        return ratio;
    }

    /// Return the point of intersection between a line and the penalty area
    ///    
    /// # Arguments
    /// - `line`: The line to check intersection with.
    /// 
    /// # Returns
    /// The point of intersection between the line and the penalty area.
    pub fn intersection_line(&self, line: Line) -> Option<Point2<f64>> {
        let (intersection, _) = self.intersection(line, false);
        return intersection;
    }

    /// Return the point of intersection between a segment and the penalty area
    /// 
    /// # Arguments
    /// - `segment`: The segment to check intersection with.
    /// 
    /// # Returns
    /// The point of intersection between the segment and the penalty area.
    pub fn intersection_segment(&self, segment: Line) -> Option<Point2<f64>> {
        let (intersection, _) = self.intersection(segment, true);
        return intersection;
    }


    /// Return a penalty enlarged with the margin parameter 
    /// 
    /// # Arguments
    /// - `offset`: The margin to add to the penalty area.
    /// 
    /// # Returns
    /// The enlarged penalty area.
    pub fn enlarged_penalty(&self, offset: f64) -> Penalty {
        let factor = self.front_line.start.x.signum();
        Penalty{
            width: self.width+offset*2.,
            depth: self.depth+offset,
            front_line: Line::new(
                Point2::new(self.front_line.start.x - factor * offset,self.front_line.start.y + factor*offset), 
                Point2::new(self.front_line.end.x - factor * offset, self.front_line.end.y - factor*offset)
            ),
            back_line: Line::new(
                Point2::new(self.back_line.start.x - factor * offset,self.back_line.start.y - factor*offset), 
                Point2::new(self.back_line.end.x - factor * offset,self.back_line.end.y + factor*offset), 
            ),
            left_line: Line::new(
                Point2::new(self.left_line.start.x,self.left_line.start.y + factor*offset), 
                Point2::new(self.left_line.end.x - factor * offset,self.left_line.end.y + factor*offset), 
            ),
            right_line: Line::new(
                Point2::new(self.right_line.start.x,self.right_line.start.y - factor*offset), 
                Point2::new(self.right_line.end.x - factor * offset,self.right_line.end.y - factor*offset), 
            ),
        }
    }

    pub fn is_inside(&self, point: Point2<f64>) -> bool {   
        self.front_line.start.x.signum() == point.x.signum()
        && self.front_line.start.x.abs() <= point.x.abs() 
        && point.y.abs() <= self.width/2.
    }
}
