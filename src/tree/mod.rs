#[cfg(test)]
pub mod test;

use flavio::math::{Tensor, TensorRank1};
use std::{
    array::from_fn,
    fs::File,
    io::{BufWriter, Error as ErrorIO, Write},
};

type Cells = [Cell; 8];
type Faces = [Option<usize>; 6];
type Indices = [usize; 8];
type OcTree = Vec<Cell>;
type Point = TensorRank1<3, 1>;
type Points = Vec<Point>;

trait Tree {
    fn balance(&mut self, levels: &usize);
    fn from_points(levels: &usize, points: &Points, foo: [f64; 6]) -> Self;
    fn subdivide(&mut self, index: usize);
    fn write_mesh(&self, file_path: &str) -> Result<(), ErrorIO>;
}

#[derive(Debug)]
struct Cell {
    cells: Option<Indices>,
    level: usize,
    faces: Faces,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
    min_z: f64,
    max_z: f64,
}

impl Cell {
    fn contains(&self, points: &Points) -> bool {
        for point in points {
            if &point[0] >= self.get_min_x()
                && &point[0] <= self.get_max_x()
                && &point[1] >= self.get_min_y()
                && &point[1] <= self.get_max_y()
                && &point[2] >= self.get_min_z()
                && &point[2] <= self.get_max_z()
            {
                return true;
            }
        }
        false
    }
    fn get_faces(&self) -> &Faces {
        &self.faces
    }
    fn get_level(&self) -> &usize {
        &self.level
    }
    fn get_min_x(&self) -> &f64 {
        &self.min_x
    }
    fn get_max_x(&self) -> &f64 {
        &self.max_x
    }
    fn get_min_y(&self) -> &f64 {
        &self.min_y
    }
    fn get_max_y(&self) -> &f64 {
        &self.max_y
    }
    fn get_min_z(&self) -> &f64 {
        &self.min_z
    }
    fn get_max_z(&self) -> &f64 {
        &self.max_z
    }
    fn subdivide(&mut self, indices: Indices) -> Cells {
        self.cells = Some(indices);
        let level = self.get_level() + 1;
        let min_x = self.get_min_x();
        let max_x = self.get_max_x();
        let min_y = self.get_min_y();
        let max_y = self.get_max_y();
        let min_z = self.get_min_z();
        let max_z = self.get_max_z();
        let val_x = 0.5 * (min_x + max_x);
        let val_y = 0.5 * (min_y + max_y);
        let val_z = 0.5 * (min_z + max_z);
        [
            Cell {
                cells: None,
                faces: [
                    None,
                    Some(indices[1]),
                    Some(indices[2]),
                    None,
                    None,
                    Some(indices[4]),
                ],
                level,
                min_x: *min_x,
                max_x: val_x,
                min_y: *min_y,
                max_y: val_y,
                min_z: *min_z,
                max_z: val_z,
            },
            Cell {
                cells: None,
                faces: [
                    None,
                    None,
                    Some(indices[3]),
                    Some(indices[0]),
                    None,
                    Some(indices[5]),
                ],
                level,
                min_x: val_x,
                max_x: *max_x,
                min_y: *min_y,
                max_y: val_y,
                min_z: *min_z,
                max_z: val_z,
            },
            Cell {
                cells: None,
                faces: [
                    Some(indices[0]),
                    Some(indices[3]),
                    None,
                    None,
                    None,
                    Some(indices[6]),
                ],
                level,
                min_x: *min_x,
                max_x: val_x,
                min_y: val_y,
                max_y: *max_y,
                min_z: *min_z,
                max_z: val_z,
            },
            Cell {
                cells: None,
                faces: [
                    Some(indices[1]),
                    None,
                    None,
                    Some(indices[2]),
                    None,
                    Some(indices[7]),
                ],
                level,
                min_x: val_x,
                max_x: *max_x,
                min_y: val_y,
                max_y: *max_y,
                min_z: *min_z,
                max_z: val_z,
            },
            Cell {
                cells: None,
                faces: [
                    None,
                    Some(indices[5]),
                    Some(indices[6]),
                    None,
                    Some(indices[0]),
                    None,
                ],
                level,
                min_x: *min_x,
                max_x: val_x,
                min_y: *min_y,
                max_y: val_y,
                min_z: val_z,
                max_z: *max_z,
            },
            Cell {
                cells: None,
                faces: [
                    None,
                    None,
                    Some(indices[7]),
                    Some(indices[6]),
                    Some(indices[1]),
                    None,
                ],
                level,
                min_x: val_x,
                max_x: *max_x,
                min_y: *min_y,
                max_y: val_y,
                min_z: val_z,
                max_z: *max_z,
            },
            Cell {
                cells: None,
                faces: [
                    Some(indices[4]),
                    Some(indices[7]),
                    None,
                    None,
                    Some(indices[2]),
                    None,
                ],
                level,
                min_x: *min_x,
                max_x: val_x,
                min_y: val_y,
                max_y: *max_y,
                min_z: val_z,
                max_z: *max_z,
            },
            Cell {
                cells: None,
                faces: [
                    Some(indices[5]),
                    None,
                    None,
                    Some(indices[6]),
                    Some(indices[3]),
                    None,
                ],
                level,
                min_x: val_x,
                max_x: *max_x,
                min_y: val_y,
                max_y: *max_y,
                min_z: val_z,
                max_z: *max_z,
            },
        ]
    }
}

impl Tree for OcTree {
    fn balance(&mut self, levels: &usize) {
        let mut index = 0;
        let mut subdivide = false;
        while index < self.len() {
            if self[index].get_level() < &(levels - 1) && self[index].cells.is_none() {
                'faces: for (face, face_cell) in self[index].get_faces().iter().enumerate() {
                    if let Some(neighbor) = face_cell {
                        if let Some(kids) = self[*neighbor].cells {
                            if match face {
                                0 => {
                                    self[kids[2]].cells.is_some()
                                        || self[kids[3]].cells.is_some()
                                        || self[kids[6]].cells.is_some()
                                        || self[kids[7]].cells.is_some()
                                }
                                1 => {
                                    self[kids[0]].cells.is_some()
                                        || self[kids[2]].cells.is_some()
                                        || self[kids[4]].cells.is_some()
                                        || self[kids[6]].cells.is_some()
                                }
                                2 => {
                                    self[kids[0]].cells.is_some()
                                        || self[kids[1]].cells.is_some()
                                        || self[kids[4]].cells.is_some()
                                        || self[kids[5]].cells.is_some()
                                }
                                3 => {
                                    self[kids[1]].cells.is_some()
                                        || self[kids[3]].cells.is_some()
                                        || self[kids[5]].cells.is_some()
                                        || self[kids[7]].cells.is_some()
                                }
                                4 => {
                                    self[kids[4]].cells.is_some()
                                        || self[kids[5]].cells.is_some()
                                        || self[kids[6]].cells.is_some()
                                        || self[kids[7]].cells.is_some()
                                }
                                5 => {
                                    self[kids[0]].cells.is_some()
                                        || self[kids[1]].cells.is_some()
                                        || self[kids[2]].cells.is_some()
                                        || self[kids[3]].cells.is_some()
                                }
                                _ => panic!(),
                            } {
                                subdivide = true;
                                break 'faces;
                            }
                        }
                    }
                }
                if subdivide {
                    self.subdivide(index);
                    subdivide = false;
                }
            }
            index += 1;
        }
    }
    fn from_points(levels: &usize, points: &Points, foo: [f64; 6]) -> Self {
        let mut tree = vec![Cell {
            cells: None,
            faces: [None; 6],
            level: 0,
            min_x: foo[0],
            max_x: foo[1],
            min_y: foo[2],
            max_y: foo[3],
            min_z: foo[4],
            max_z: foo[5],
        }];
        //
        // find bounds yourself instead of using foo
        //
        let mut index = 0;
        while index < tree.len() {
            if tree[index].get_level() < levels && tree[index].contains(points) {
                tree.subdivide(index);
            }
            index += 1;
        }
        tree
    }
    fn subdivide(&mut self, index: usize) {
        let new_indices = from_fn(|n| self.len() + n);
        let mut new_cells = self[index].subdivide(new_indices);
        self[index]
            .get_faces()
            .clone()
            .iter()
            .enumerate()
            .for_each(|(face, face_cell)| {
                if let Some(neighbor) = face_cell {
                    if let Some(kids) = self[*neighbor].cells {
                        match face {
                            0 => {
                                new_cells[0].faces[0] = Some(kids[2]);
                                new_cells[1].faces[0] = Some(kids[3]);
                                new_cells[4].faces[0] = Some(kids[6]);
                                new_cells[5].faces[0] = Some(kids[7]);
                                self[kids[2]].faces[2] = Some(new_indices[0]);
                                self[kids[3]].faces[2] = Some(new_indices[1]);
                                self[kids[6]].faces[2] = Some(new_indices[4]);
                                self[kids[7]].faces[2] = Some(new_indices[5]);
                            }
                            1 => {
                                new_cells[1].faces[1] = Some(kids[0]);
                                new_cells[3].faces[1] = Some(kids[2]);
                                new_cells[5].faces[1] = Some(kids[4]);
                                new_cells[7].faces[1] = Some(kids[6]);
                                self[kids[0]].faces[3] = Some(new_indices[1]);
                                self[kids[2]].faces[3] = Some(new_indices[3]);
                                self[kids[4]].faces[3] = Some(new_indices[5]);
                                self[kids[6]].faces[3] = Some(new_indices[7]);
                            }
                            2 => {
                                new_cells[2].faces[2] = Some(kids[0]);
                                new_cells[3].faces[2] = Some(kids[1]);
                                new_cells[6].faces[2] = Some(kids[4]);
                                new_cells[7].faces[2] = Some(kids[5]);
                                self[kids[0]].faces[0] = Some(new_indices[2]);
                                self[kids[1]].faces[0] = Some(new_indices[3]);
                                self[kids[4]].faces[0] = Some(new_indices[6]);
                                self[kids[5]].faces[0] = Some(new_indices[7]);
                            }
                            3 => {
                                new_cells[0].faces[3] = Some(kids[1]);
                                new_cells[2].faces[3] = Some(kids[3]);
                                new_cells[4].faces[3] = Some(kids[5]);
                                new_cells[6].faces[3] = Some(kids[7]);
                                self[kids[1]].faces[1] = Some(new_indices[0]);
                                self[kids[3]].faces[1] = Some(new_indices[2]);
                                self[kids[5]].faces[1] = Some(new_indices[4]);
                                self[kids[7]].faces[1] = Some(new_indices[6]);
                            }
                            4 => {
                                new_cells[0].faces[4] = Some(kids[4]);
                                new_cells[1].faces[4] = Some(kids[5]);
                                new_cells[2].faces[4] = Some(kids[6]);
                                new_cells[3].faces[4] = Some(kids[7]);
                                self[kids[4]].faces[5] = Some(new_indices[0]);
                                self[kids[5]].faces[5] = Some(new_indices[1]);
                                self[kids[6]].faces[5] = Some(new_indices[2]);
                                self[kids[7]].faces[5] = Some(new_indices[3]);
                            }
                            5 => {
                                new_cells[4].faces[5] = Some(kids[0]);
                                new_cells[5].faces[5] = Some(kids[1]);
                                new_cells[6].faces[5] = Some(kids[2]);
                                new_cells[7].faces[5] = Some(kids[3]);
                                self[kids[0]].faces[4] = Some(new_indices[4]);
                                self[kids[1]].faces[4] = Some(new_indices[5]);
                                self[kids[2]].faces[4] = Some(new_indices[6]);
                                self[kids[3]].faces[4] = Some(new_indices[7]);
                            }
                            _ => panic!(),
                        }
                    }
                }
            });
        self.extend(new_cells);
    }
    fn write_mesh(&self, file_path: &str) -> Result<(), ErrorIO> {
        let mesh_file = File::create(file_path)?;
        let mut file = BufWriter::new(mesh_file);
        file.write_all(b"MeshVersionFormatted 1\nDimension 3\nVertices\n")?;

        let num_cells = self.iter().filter(|cell| cell.cells.is_none()).count();

        file.write_all(format!("{}\n", num_cells * 8).as_bytes())?;

        let mut nodal_coordinates = [
            Point::zero(),
            Point::zero(),
            Point::zero(),
            Point::zero(),
            Point::zero(),
            Point::zero(),
            Point::zero(),
            Point::zero(),
        ];
        self.iter()
            .filter(|cell| cell.cells.is_none())
            .try_for_each(|cell| {
                nodal_coordinates[0] = Point::new([
                    cell.get_min_x().copy(),
                    cell.get_min_y().copy(),
                    cell.get_min_z().copy(),
                ]);
                nodal_coordinates[1] = Point::new([
                    cell.get_max_x().copy(),
                    cell.get_min_y().copy(),
                    cell.get_min_z().copy(),
                ]);
                nodal_coordinates[2] = Point::new([
                    cell.get_max_x().copy(),
                    cell.get_max_y().copy(),
                    cell.get_min_z().copy(),
                ]);
                nodal_coordinates[3] = Point::new([
                    cell.get_min_x().copy(),
                    cell.get_max_y().copy(),
                    cell.get_min_z().copy(),
                ]);
                nodal_coordinates[4] = Point::new([
                    cell.get_min_x().copy(),
                    cell.get_min_y().copy(),
                    cell.get_max_z().copy(),
                ]);
                nodal_coordinates[5] = Point::new([
                    cell.get_max_x().copy(),
                    cell.get_min_y().copy(),
                    cell.get_max_z().copy(),
                ]);
                nodal_coordinates[6] = Point::new([
                    cell.get_max_x().copy(),
                    cell.get_max_y().copy(),
                    cell.get_max_z().copy(),
                ]);
                nodal_coordinates[7] = Point::new([
                    cell.get_min_x().copy(),
                    cell.get_max_y().copy(),
                    cell.get_max_z().copy(),
                ]);
                nodal_coordinates.iter().try_for_each(|coordinates| {
                    coordinates.iter().try_for_each(|coordinate| {
                        file.write_all(format!("{} ", coordinate).as_bytes())
                    })?;
                    file.write_all(b"0\n")
                })
            })?;

        file.write_all(b"Hexahedra\n")?;

        file.write_all(format!("{}\n", num_cells).as_bytes())?;

        let mut index = 0;
        let mut connectivity = [0; 8];
        self.iter()
            .filter(|cell| cell.cells.is_none())
            .try_for_each(|_| {
                connectivity = from_fn(|n| index + n);
                index += 8;
                connectivity
                    .iter()
                    .try_for_each(|node| file.write_all(format!("{} ", node + 1).as_bytes()))?;
                file.write_all(b"0\n")
            })?;

        file.write_all(b"End")?;
        file.flush()
    }
}
