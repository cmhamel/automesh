use super::{ErrorIO, OcTree, Point, Points, Tree};
use flavio::math::Tensor;

#[test]
fn foobar() -> Result<(), ErrorIO> {
    let levels = 5;
    let points = vec![Point::new([0.499, 0.499, 0.499])];
    let mut tree = OcTree::from_points(&levels, &points, [0.0, 1.0, 0.0, 1.0, 0.0, 1.0]);
    tree.balance(&levels);
    tree.balance(&levels);
    tree.balance(&levels);
    tree.balance(&levels);
    tree.write_mesh("primal.mesh")
}
