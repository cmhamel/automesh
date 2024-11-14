use flavio::math::{Tensor, TensorRank1};

type Point = TensorRank1<2, 1>;
type Points = Vec<Point>;

#[derive(Debug)]
struct Cell {
    fac_u: Vec<usize>,
    fac_d: Vec<usize>,
    fac_l: Vec<usize>,
    fac_r: Vec<usize>,
    level: usize,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Cell {
    fn contains(&self, points: &Points) -> bool {
        for point in points {
            if &point[0] >= self.get_min_x()
                && &point[0] <= self.get_max_x()
                && &point[1] >= self.get_min_y()
                && &point[1] <= self.get_max_y()
            {
                return true;
            }
        }
        false
    }
    fn get_level(&self) -> &usize {
        &self.level
    }
    fn get_min_x(&self) -> &f64 {
        &self.min_x
    }
    fn get_min_y(&self) -> &f64 {
        &self.min_y
    }
    fn get_max_x(&self) -> &f64 {
        &self.max_x
    }
    fn get_max_y(&self) -> &f64 {
        &self.max_y
    }
}

const X_MIN: f64 = 0.0;
const X_MAX: f64 = 8.0;
const Y_MIN: f64 = 0.0;
const Y_MAX: f64 = 8.0;

#[test]
fn foo() {
    let m = 6;
    let points = vec![
        Point::new([1.2, 3.3]),
        Point::new([5.2, 2.3]),
        Point::new([6.6, 6.6]),
    ];
    let mut tree = vec![
        Cell {
            fac_u: vec![2],
            fac_d: vec![],
            fac_l: vec![],
            fac_r: vec![1],
            level: 1,
            min_x: X_MIN,
            min_y: Y_MIN,
            max_x: 0.5 * (X_MIN + X_MAX),
            max_y: 0.5 * (Y_MIN + Y_MAX),
        },
        Cell {
            fac_u: vec![3],
            fac_d: vec![],
            fac_l: vec![0],
            fac_r: vec![],
            level: 1,
            min_x: 0.5 * (X_MIN + X_MAX),
            min_y: Y_MIN,
            max_x: X_MAX,
            max_y: 0.5 * (Y_MIN + Y_MAX),
        },
        Cell {
            fac_u: vec![],
            fac_d: vec![0],
            fac_l: vec![],
            fac_r: vec![3],
            level: 1,
            min_x: X_MIN,
            min_y: 0.5 * (Y_MIN + Y_MAX),
            max_x: 0.5 * (X_MIN + X_MAX),
            max_y: Y_MAX,
        },
        Cell {
            fac_u: vec![],
            fac_d: vec![1],
            fac_l: vec![2],
            fac_r: vec![],
            level: 1,
            min_x: 0.5 * (X_MIN + X_MAX),
            min_y: 0.5 * (Y_MIN + Y_MAX),
            max_x: X_MAX,
            max_y: Y_MAX,
        },
    ];
    let mut cell;
    let mut index = 0;
    let mut level;
    let mut min_x;
    let mut min_y;
    let mut max_x;
    let mut max_y;
    let mut val_x;
    let mut val_y;
    while index < tree.len() {
        cell = &tree[index];
        if cell.get_level() < &m && cell.contains(&points) {
            level = cell.get_level() + 1;
            min_x = cell.get_min_x();
            min_y = cell.get_min_y();
            max_x = cell.get_max_x();
            max_y = cell.get_max_y();
            val_x = 0.5 * (min_x + max_x);
            val_y = 0.5 * (min_y + max_y);
            //
            // (1) if you remove the parent cell at the end, you are changing the cell numbering for cells after!
            // (2) new cell numbers are easy since they fit at end of vec
            // (3) for each neighbor of cell, need to replace cell in the neighbor list by some subset of below
            // (4) fill out neighbors of new cells conversely
            //
            tree.extend([
                Cell {
                    fac_u: vec![],
                    fac_d: vec![],
                    fac_l: vec![],
                    fac_r: vec![],
                    level,
                    min_x: *min_x,
                    min_y: *min_y,
                    max_x: val_x,
                    max_y: val_y,
                },
                Cell {
                    fac_u: vec![],
                    fac_d: vec![],
                    fac_l: vec![],
                    fac_r: vec![],
                    level,
                    min_x: val_x,
                    min_y: *min_y,
                    max_x: *max_x,
                    max_y: val_y,
                },
                Cell {
                    fac_u: vec![],
                    fac_d: vec![],
                    fac_l: vec![],
                    fac_r: vec![],
                    level,
                    min_x: *min_x,
                    min_y: val_y,
                    max_x: val_x,
                    max_y: *max_y,
                },
                Cell {
                    fac_u: vec![],
                    fac_d: vec![],
                    fac_l: vec![],
                    fac_r: vec![],
                    level,
                    min_x: val_x,
                    min_y: val_y,
                    max_x: *max_x,
                    max_y: *max_y,
                },
            ]);
            tree.remove(index);
            // would keeping the "parent" cells help with balancing somehow?
            // you could store "garbage" indices and remove them from the tree later (take out the garbage)
            // or maybe you can move it into another Vec and loop over those during the balancing part?
            // or use the coordinates of a cell and contains() for all other cells? that would be slow, though.
            // or construct some sort of neighboring list on-the-fly to check against?
            // wont you need some sort of neighbor list for the primal-to-dual transition too?
            //
            // do you want to template the Tree to change NSD based on NSD in the input points? might help with testing
            //
            // during balancing, you can at least rule out further subdividing the smallest 2 sizes:
            // loop through level 1's => subdivide if any neighbors level 3 or below
            // loop through level 2's => subdivide if any neighbors level 4 or below
            // repeat until hit level (m - 1)
            //
        } else {
            index += 1;
        }
    }
    tree.iter().for_each(|cell| println!("{:?}", cell));
    tree.iter().for_each(|cell| {
        println!(
            "ax.add_patch(patches.Rectangle(({},{}),{},{}, edgecolor='red'))",
            cell.min_x,
            cell.min_y,
            cell.max_x - cell.min_x,
            cell.max_y - cell.min_y
        )
    })
}
