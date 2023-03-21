use std::collections::HashMap;
use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use crabe_math::shape::{Circle, Line, Rectangle};
use serde_with::serde_as;

#[serde_as]
#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnotationStore {
    #[serde_as(as = "Vec<(_, _)>")]
    annotations: HashMap<String, Annotation>
}

///
#[derive(Clone, Serialize)]
#[serde(tag = "kind", content = "content", rename_all = "camelCase")]
pub enum Annotation {
    Circle(Circle),
    Line(Line),
    Rectangle(Rectangle),
    Point(Point2<f64>),
}

impl AnnotationStore {
    /// Add a circle annotation
    ///
    /// # Arguments
    ///
    /// * `id`: Annotation id
    /// * `circle`: Circle to add
    ///
    /// returns: ()
    pub fn add_circle(&mut self, id: String, circle: Circle) {
        self.annotations.insert(id, Annotation::Circle(circle));
    }

    /// Add a point annotation
    ///
    /// # Arguments
    ///
    /// * `id`: Annotation id
    /// * `point`: Point to add
    ///
    /// returns: ()
    pub fn add_point(&mut self, id: String, point: Point2<f64>) {
        self.annotations.insert(id, Annotation::Point(point));
    }

    /// Add a rectangle annotation
    ///
    /// # Arguments
    ///
    /// * `id`: Annotation id
    /// * `rectangle`: Rectangle to add
    ///
    /// returns: ()
    pub fn add_rectangle(&mut self, id: String, rectangle: Rectangle) {
        self.annotations.insert(id, Annotation::Rectangle(rectangle));
    }

    /// Clear annotations
    ///
    /// returns: ()
    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    /// Get an annotation
    ///
    /// # Arguments
    ///
    /// * `id`: Annotation id
    ///
    /// returns: Option<&Annotation>
    pub fn get(&self, id: &str) -> Option<&Annotation> {
        self.annotations.get(id)
    }
}