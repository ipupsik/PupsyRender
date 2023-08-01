use crate::engine::geometry::triangle::*;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug, Default, Hash)]
pub(crate) struct OctantId(pub usize);

#[derive(Clone, Debug, Default)]
/// A specific node in octree
pub(crate) struct Octant {
    /// Tree related attributes. For root octant, the parent is None.
    pub parent: Option<OctantId>,
    pub children: Vec<OctantId>,

    /// The actual data which will be stored within the tree.
    pub center: Triangle,
    /// The extent of octant (in radius).
    pub extent: f64,
    /// Child point indices in point cloud.
    pub ipoints: Vec<usize>,
    /// The ranking within sibling octants.
    pub ranking: usize,
}

impl Octant {
    /// Construct an Octant cube with extent (in radius).
    ///
    /// # Panic
    ///
    /// * Panics if extent is negative.
    ///
    pub fn new(extent: f64) -> Self {
        assert!(extent.is_sign_positive());
        Octant {
            extent: extent,
            ..Default::default()
        }
    }

    /// Construct a root octant from 3D points
    pub fn from_points(points: &[Triangle]) -> Self {
        // define the boundary in XYZ directions
        let xs: Vec<_> = points.iter().map(|[x, _, _]| *x).collect();
        let ys: Vec<_> = points.iter().map(|[_, y, _]| *y).collect();
        let zs: Vec<_> = points.iter().map(|[_, _, z]| *z).collect();

        let (xmin, ymin, zmin) = (xs.min(), ys.min(), zs.min());
        let (xmax, ymax, zmax) = (xs.max(), ys.max(), zs.max());

        let (wx, wy, wz) = (xmax - xmin, ymax - ymin, zmax - zmin);
        let width = [wx, wy, wz].max();

        // Construct the root octant containg all points
        let mut o = Octant::new(0.5 * width);
        o.center = [(xmax + xmin) / 2., (ymax + ymin) / 2., (zmax + zmin) / 2.];

        o.ipoints = (0..points.len()).collect();
        o
    }

    /// test if two octants are neighboring
    pub fn neighboring(&self, other: &Octant) -> bool {
        let e = other.extent + self.extent;

        for i in 0..3 {
            let v = (other.center[i] - self.center[i]).abs() - e;
            if v > 0.001 {
                return false;
            }
        }

        true
    }
}