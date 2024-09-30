use super::{
    finite_element_data_from_npy_data, voxel_data_from_npy, voxel_data_from_spn,
    write_voxels_to_npy, Nel, Scale, Translate, VoxelData,
};
use crate::fem::py::FiniteElements;
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
    /// Converts the voxels type into a finite elements type.
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
            data: voxel_data_from_spn(file_path, nel)
                .expect("error reading voxels data from SPN file"),
        }
    }
    /// Writes the internal voxels data to an NPY file.
    pub fn write_npy(&self, file_path: &str) {
        write_voxels_to_npy(&self.data, file_path).expect("error writing voxels data to NPY file")
    }
}
