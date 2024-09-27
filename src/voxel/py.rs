use super::{
    finite_element_data_from_npy_data, voxel_data_from_npy, voxel_data_from_spn,
    write_voxels_to_npy, Nel, Scale, Translate, VoxelData,
};
use crate::fem::py::FiniteElements;
use numpy::{PyArray3, ToPyArray};
use pyo3::prelude::*;

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Voxels>()?;
    Ok(())
}

/// The voxels class.
#[pyclass]
pub struct Voxels {
    data: VoxelData,
}

#[pymethods]
impl Voxels {
    #[doc = svgbobdoc::transform!(
    /// Converts the voxels type into a finite element type.
    ///
    /// The voxel data can be scaled and translated (in that order).
    ///
    /// $$
    /// x \mapsto s_x x + t_x\qquad y \mapsto s_y y + t_y\qquad z \mapsto s_z z + t_z
    /// $$
    ///
    /// ```svgbob
    ///                     8       7
    ///                      *-------*
    ///                   5 /|    6 /|
    ///  z                 *-+-----* |
    ///   ^  y             | |4    | |3
    ///   | ^              | *-----|-*
    ///   |/               |/      |/
    ///   +-----> x        *-------*
    ///                    1       2
    /// ```
    )]
    #[pyo3(signature = (remove=[0].to_vec(), scale=[1.0, 1.0, 1.0], translate=[0.0, 0.0, 0.0]))]
    pub fn as_finite_elements(
        &self,
        remove: Option<Vec<u8>>,
        scale: Scale,
        translate: Translate,
    ) -> FiniteElements {
        let (element_blocks, element_node_connectivity, nodal_coordinates) =
            finite_element_data_from_npy_data(&self.data, remove, &scale, &translate);
        FiniteElements::from_data(element_blocks, element_node_connectivity, nodal_coordinates)
    }
    /// The internal voxels data.
    #[getter]
    pub fn get_data<'py>(&self, python: Python<'py>) -> Bound<'py, PyArray3<u8>> {
        self.data.to_pyarray_bound(python)
    }
    /// Constructs and returns a new voxels type from an NPY file.
    #[staticmethod]
    pub fn from_npy(file_path: &str) -> Self {
        Self {
            data: voxel_data_from_npy(file_path).expect("error reading voxels data from NPY file"),
        }
    }
    /// Constructs and returns a new voxels type from an SPN file.
    #[staticmethod]
    pub fn from_spn(file_path: &str, nel: Nel) -> Self {
        Self {
            data: voxel_data_from_spn(file_path, nel),
        }
    }
    /// Writes the internal voxels data to an NPY file.
    pub fn write_npy(&self, file_path: &str) {
        write_voxels_to_npy(&self.data, file_path);
    }
}
