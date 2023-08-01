type Point = [f64; 3];

use super::octant::*;

#[derive(Debug)]
pub(crate) struct Query {
    pub center: Point,
    pub radius: f64,
}

/// Four possible relations of a query ball with an octant
#[derive(Debug, PartialEq)]
pub(crate) enum QORelation {
    /// the query ball has no common space with the octant
    Disjoint,
    /// the query ball is partially overlapping with the octant
    Overlaps,
    /// the query ball completely contains the octant
    Contains,
    /// the query ball is completely within the octant
    Within,
}

impl Query {
    pub fn new(r: f64) -> Self {
        assert!(r.is_sign_positive(), "radius has to be positive: {}", r);
        Query {
            center: [0.0; 3],
            radius: r,
        }
    }

    /// calculate the relation of query ball with octant
    pub fn relation(&self, octant: &Octant) -> QORelation {
        let extent = octant.extent;
        let radius = self.radius;

        let x = (self.center[0] - octant.center[0]).abs();
        let y = (self.center[1] - octant.center[1]).abs();
        let z = (self.center[2] - octant.center[2]).abs();

        // 1. cheap case: xyz > e+r
        let max_dist = extent + radius;
        if x > max_dist || y > max_dist || z > max_dist {
            return QORelation::Disjoint;
        }

        // 2. overlaps or not
        if x < extent || y < extent || z < extent {
            // expected to be common: e >= r
            // expected to be rare  : e < r
            if extent >= radius {
                // 2.1 Within
                // cheap case: xyz < e-r < e+r
                let min_dist = extent - radius;
                if x <= min_dist && y <= min_dist && z <= min_dist {
                    return QORelation::Within;
                }
            } else {
                if x <= extent && y <= extent && z <= extent {
                    // distance to the farthest corner point
                    let r_sqr = radius * radius;
                    let e = extent;
                    let d_sqr = (x + e) * (x + e) + (y + e) * (y + e) + (z + e) * (z + e);
                    // 2.2 Contains
                    if d_sqr <= r_sqr {
                        return QORelation::Contains;
                    }
                }
            }
            // cheap case: e < x < e+r || e < y < e+r || z < e < e+r
            return QORelation::Overlaps;
        }

        // 3. corner case: Disjoint or Overlaps?
        // FIXME: can we just assume "Overlaps" to improve efficiency?
        // expensive case: e < xyz < e+r
        // distance to the nearest corner point
        let r_sqr = radius * radius;
        let e = extent;
        let d_sqr = (x - e) * (x - e) + (y - e) * (y - e) + (z - e) * (z - e);
        if d_sqr > r_sqr {
            return QORelation::Disjoint;
        }

        QORelation::Overlaps
    }
}