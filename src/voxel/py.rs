use super::{
    finite_element_data_from_data, voxel_data_from_npy, voxel_data_from_spn, write_voxels_to_npy,
    write_voxels_to_spn, IntermediateError, Nel, Scale, Translate, VoxelData,
};
use crate::fem::py::FiniteElements;
use ndarray_npy::{ReadNpyError, WriteNpyError};
use pyo3::{exceptions::PyTypeError, prelude::*};
use std::{convert::From, io::Error};

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
    ) -> Result<FiniteElements, PyIntermediateError> {
        let (element_blocks, element_node_connectivity, nodal_coordinates) =
            finite_element_data_from_data(&self.data, remove, &scale, &translate)?;
        Ok(FiniteElements::from_data(
            element_blocks,
            element_node_connectivity,
            nodal_coordinates,
        ))
    }
    /// Constructs and returns a new voxels type from an NPY file.
    #[staticmethod]
    pub fn from_npy(file_path: &str) -> Result<Self, PyIntermediateError> {
        Ok(Self {
            data: voxel_data_from_npy(file_path)?,
        })
    }
    /// Constructs and returns a new voxels type from an SPN file.
    #[staticmethod]
    pub fn from_spn(file_path: &str, nel: Nel) -> Result<Self, PyIntermediateError> {
        Ok(Self {
            data: voxel_data_from_spn(file_path, nel)?,
        })
    }
    /// Writes the internal voxels data to an NPY file.
    pub fn write_npy(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_voxels_to_npy(&self.data, file_path)?)
    }
    /// Writes the internal voxels data to an SPN file.
    pub fn write_spn(&self, file_path: &str) -> Result<(), PyIntermediateError> {
        Ok(write_voxels_to_spn(&self.data, file_path)?)
    }
}

pub struct PyIntermediateError {
    message: String,
}

impl From<Error> for PyIntermediateError {
    fn from(error: Error) -> PyIntermediateError {
        PyIntermediateError {
            message: error.to_string(),
        }
    }
}

impl From<ReadNpyError> for PyIntermediateError {
    fn from(error: ReadNpyError) -> PyIntermediateError {
        PyIntermediateError {
            message: error.to_string(),
        }
    }
}

impl From<String> for PyIntermediateError {
    fn from(message: String) -> PyIntermediateError {
        PyIntermediateError { message }
    }
}

impl From<WriteNpyError> for PyIntermediateError {
    fn from(error: WriteNpyError) -> PyIntermediateError {
        PyIntermediateError {
            message: error.to_string(),
        }
    }
}

impl From<PyIntermediateError> for PyErr {
    fn from(error: PyIntermediateError) -> PyErr {
        PyTypeError::new_err(error.message)
    }
}

impl From<IntermediateError> for PyIntermediateError {
    fn from(error: IntermediateError) -> PyIntermediateError {
        PyIntermediateError {
            message: error.message,
        }
    }
}
