use flavio::math::{Tensor, TensorRank1};

type Point = TensorRank1<2, 1>;

#[derive(Debug)]
struct Cell {
    level: usize,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Cell {
    fn contains(&self, point: &Point) -> bool {
        &point[0] >= self.get_min_x()
            && &point[0] <= self.get_max_x()
            && &point[1] >= self.get_min_y()
            && &point[1] <= self.get_max_y()
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

#[test]
fn foo() {
    let m = 3;
    let point = Point::new([6.6, 6.6]);
    let mut tree = vec![Cell {
        level: 0,
        min_x: 0.0,
        min_y: 0.0,
        max_x: 8.0,
        max_y: 8.0,
    }];
    // let mut tree_iter = tree.iter_mut();
    // let mut foo = tree_iter.next();
    // let mut index = 0;
    // while let Some(cell) = foo {
    //     if cell.level < m && cell.contains(&point) {
    //         tree.remove(index);
    //     } else {
    //         foo = tree_iter.next();
    //         index += 1;
    //     }
    // }
    let mut cell;
    let mut index = 0;
    let mut level;
    let mut min_x;
    let mut min_y;
    let mut max_x;
    let mut max_y;
    while index < tree.len() {
        cell = &tree[index];
        if cell.get_level() < &m && cell.contains(&point) {
            level = cell.get_level() + 1;
            min_x = cell.get_min_x();
            min_y = cell.get_min_y();
            max_x = cell.get_max_x();
            max_y = cell.get_max_y();
            tree.extend([
                Cell {
                    level,
                    min_x: *min_x,
                    min_y: *min_y,
                    max_x: 0.5 * (min_x + max_x),
                    max_y: 0.5 * (min_y + max_y),
                },
                Cell {
                    level,
                    min_x: 0.5 * (min_x + max_x),
                    min_y: *min_y,
                    max_x: *max_x,
                    max_y: 0.5 * (min_y + max_y),
                },
                Cell {
                    level,
                    min_x: *min_x,
                    min_y: 0.5 * (min_y + max_y),
                    max_x: 0.5 * (min_x + max_x),
                    max_y: *max_y,
                },
                Cell {
                    level,
                    min_x: 0.5 * (min_x + max_x),
                    min_y: 0.5 * (min_y + max_y),
                    max_x: *max_x,
                    max_y: *max_y,
                },
            ]);
            tree.remove(index);
        } else {
            index += 1;
        }
        println!("{:?}", tree);
    }
}
