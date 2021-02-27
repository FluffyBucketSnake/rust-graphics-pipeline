use crate::vertex::Vertex;

#[derive(Clone, Copy)]
pub struct Line<T>(pub T, pub T);

#[derive(Clone, Copy)]
pub struct Triangle<T>(pub T, pub T, pub T);

pub enum WindingOrder {
    CounterClockwise,
    Clockwise,
    Both,
}

impl Triangle<Vertex> {
    pub fn order(&self) -> WindingOrder {
        let d1 = self.2.position - self.1.position;
        let d2 = self.1.position - self.0.position;

        let v = -self.0.position;
        let w = d1.cross(&d2);

        let dot = v.dot(&w);
        if dot > 0.0 { WindingOrder::CounterClockwise }
        else if dot < 0.0 { WindingOrder::Clockwise }
        else { WindingOrder::Both }
    }
}
