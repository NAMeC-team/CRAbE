use std::collections::HashMap;
use nalgebra::Point2;
use serde::{Deserialize, Serialize};
use crabe_math::shape::{Circle, Line, Rectangle};

#[derive(Clone, Default, Serialize)]
pub struct AnnotationStore {
    annotations: HashMap<String, Annotation>
}

#[derive(Clone, Serialize)]
pub enum Annotation {
    Circle(Circle),
    Line(Line),
    Rectangle(Rectangle),
    Point(Point2<f64>),
}

impl AnnotationStore {
    pub fn add_circle(&mut self, id: String, circle: Circle) {
        self.annotations.insert(id, Annotation::Circle(circle));
    }

    pub fn add_point(&mut self, id: String, point: Point2<f64>) {
        self.annotations.insert(id, Annotation::Point(point));
    }

    pub fn add_rectangle(&mut self, id: String, rectangle: Rectangle) {
        self.annotations.insert(id, Annotation::Rectangle(rectangle));
    }

    pub fn clear(&mut self) {
        self.annotations.clear();
    }

    pub fn get(&self, id: &str) -> Option<&Annotation> {
        self.annotations.get(id)
    }
}