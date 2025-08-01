use super::Intersects;
use crate::*;

// Blanket implementation from Coord<T>
impl<T, G> Intersects<G> for Point<T>
where
    T: CoordNum,
    Coord<T>: Intersects<G>,
{
    fn intersects(&self, rhs: &G) -> bool {
        self.0.intersects(rhs)
    }
}

// Blanket implementation from Point<T>
impl<T, G> Intersects<G> for MultiPoint<T>
where
    T: CoordNum,
    Point<T>: Intersects<G>,
{
    fn intersects(&self, rhs: &G) -> bool {
        self.iter().any(|p| p.intersects(rhs))
    }
}
