use crate::fem::tri::NUM_NODES_TRI;

/// Goals:
/// Make tessellations create 3D tri meshes,
/// similar to how voxels create 3D hex meshes.
/// Just as voxels are not hexes, tessellations (stl file) are not 3D triangles.
///
/// Read in the stl, save to Tessellation type, and then write out as a stl (stl -> stl)
/// Smooth an stl and write out a smoothed stl (stl -> stl')
/// Convert the stl into a 3D triangular finite element mesh (stl -> inp)

/// Reference:
/// https://github.com/hmeyer/stl_io
use super::{FiniteElements, TriangularFiniteElements, Vector, fem::NODE_NUMBERING_OFFSET};
use conspire::math::{Tensor, TensorArray};
use std::fmt;
use std::fs::File;
use std::io::{BufWriter, Error};
// use std::ops::Index;
// use std::io::{self, Write};
// use std::path::Path;
use stl_io::{read_stl, write_stl, IndexedMesh, IndexedTriangle, Normal, Triangle, Vertex};

/// The tessellation type.
#[derive(Debug, PartialEq)]
pub struct Tessellation {
    data: IndexedMesh,
}

/// Inherent implementation of the tessellation type
impl Tessellation {
    /// Construct a tessellation from an IndexedMesh.
    pub fn new(indexed_mesh: IndexedMesh) -> Self {
        Self { data: indexed_mesh }
    }
    /// Constructs a tessellation from finite elements, consuming the finite elements.
    pub fn from_finite_elements(finite_elements: TriangularFiniteElements) -> Self {
        let mut normal = Vector::zero();
        let mut vertices_tri = [0; NUM_NODES_TRI];
        let nodal_coordinates = finite_elements.get_nodal_coordinates();
        let vertices = nodal_coordinates
            .iter()
            .map(|coordinate| {
                Vertex::new([
                    coordinate[0] as f32,
                    coordinate[1] as f32,
                    coordinate[2] as f32,
                ])
            })
            .collect();
        let faces = finite_elements
            .get_element_node_connectivity()
            .iter()
            .map(|&connectivity| {
                vertices_tri = [connectivity[0] - NODE_NUMBERING_OFFSET, connectivity[1] - NODE_NUMBERING_OFFSET, connectivity[2] - NODE_NUMBERING_OFFSET];
                normal = (&nodal_coordinates[vertices_tri[1]]
                    - &nodal_coordinates[vertices_tri[0]])
                    .cross(
                        &(&nodal_coordinates[vertices_tri[2]]
                            - &nodal_coordinates[vertices_tri[0]]),
                    )
                    .normalized();
                IndexedTriangle {
                    normal: Normal::new([normal[0] as f32, normal[1] as f32, normal[2] as f32]),
                    vertices: vertices_tri,
                }
            })
            .collect();
        Self {
            data: IndexedMesh { vertices, faces },
        }
    }
    /// Constructs and returns a new tessellation type from a STL file.
    pub fn from_stl(file_path: &str) -> Self {
        let mut file = File::open(file_path).expect("Failed to open STL file.");
        let data: IndexedMesh = read_stl(&mut file).expect("Failed to read STL file.");
        Self { data }
    }
    /// Returns a reference to the internal tessellation data.
    pub fn get_data(&self) -> &IndexedMesh {
        &self.data
    }
    /// Write the internal tessellation data to a binary STL file.
    pub fn write_stl(&self, file_path: &str) -> Result<(), Error> {
        write_tessellation_to_stl(self.get_data(), file_path)
    }
}

fn write_tessellation_to_stl(data: &IndexedMesh, file_path: &str) -> Result<(), Error> {
    let mut file = BufWriter::new(File::create(file_path)?);
    let mesh_iter = data.faces.iter().map(|face| Triangle {
                    normal: face.normal,
                    vertices: face
                        .vertices
                        .iter()
                        .map(|&vertex| data.vertices[vertex])
                        .collect::<Vec<Vertex>>()
                        .try_into()
                        .unwrap(),
                });
    write_stl(&mut file, mesh_iter)?;
    Ok(())
}

// Implement Display trait for better debugging output
impl fmt::Display for Tessellation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Tessellation with {} vertices and {} faces",
            self.data.vertices.len(),
            self.data.faces.len()
        )
    }
}
