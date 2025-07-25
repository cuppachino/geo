use super::{has_disjoint_bboxes, Intersects};
use crate::BoundingRect;
use crate::*;

// Blanket implementation using self.lines().any().
impl<T, G> Intersects<G> for LineString<T>
where
    T: CoordNum,
    Line<T>: Intersects<G>,
    G: BoundingRect<T>,
{
    fn intersects(&self, geom: &G) -> bool {
        if has_disjoint_bboxes(self, geom) {
            return false;
        }
        self.lines().any(|l| l.intersects(geom))
    }
}

// Blanket implementation from LineString<T>
impl<T, G> Intersects<G> for MultiLineString<T>
where
    T: CoordNum,
    LineString<T>: Intersects<G>,
    G: BoundingRect<T>,
{
    fn intersects(&self, rhs: &G) -> bool {
        if has_disjoint_bboxes(self, rhs) {
            return false;
        }
        self.iter().any(|p| p.intersects(rhs))
    }
}
