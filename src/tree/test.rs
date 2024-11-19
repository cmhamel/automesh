use super::{ErrorIO, OcTree, Point, Tree};
use flavio::math::Tensor;

#[test]
fn foobar() -> Result<(), ErrorIO> {
    let levels = 9;
    let points = vec![
        Point::new([0.0001, 0.0001, 0.0001]),
        Point::new([0.4999, 0.4999, 0.4999]),
        Point::new([0.9999, 0.9999, 0.9999]),
    ];
    let mut tree = OcTree::from_points(&levels, &points);
    tree.balance(&levels);
    tree.write_mesh("primal.mesh")
}
