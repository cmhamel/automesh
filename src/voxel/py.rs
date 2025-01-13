use super::{
    super::{
        fem::py::FiniteElements,
        py::{IntoFoo, PyIntermediateError},
        Blocks,
    },
    defeature_voxels, finite_element_data_from_data, voxel_data_from_npy, voxel_data_from_spn,
    write_voxels_to_npy, write_voxels_to_spn, Nel, Vector, VoxelData,
};
use conspire::math::TensorArray;
use pyo3::{PyClass, prelude::*, pyclass::boolean_struct};

pub fn register_module(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    parent_module.add_class::<Voxels>()?;
    Ok(())
}

/// The voxels class.
#[pyclass]
pub struct Voxels {
    data: VoxelData,
}

unsafe impl pyo3::type_object::PyTypeInfo for Nel {
    const NAME: &'static str = "Nel";
    const MODULE: ::std::option::Option<&'static str> = ::std::option::Option::None;
    #[inline]
    fn type_object_raw(py: pyo3::Python<'_>) -> *mut pyo3::ffi::PyTypeObject {
        <Self as pyo3::impl_::pyclass::PyClassImpl>::lazy_type_object()
            .get_or_init(py)
            .as_type_ptr()
    }
}

impl PyClass for Nel {
    type Frozen = boolean_struct::True;
}

#[pymethods]
impl Voxels {
    /// Converts the voxels type into a finite elements type.
    #[pyo3(signature = (remove=[].to_vec(), scale=[1.0, 1.0, 1.0], translate=[0.0, 0.0, 0.0]))]
    pub fn as_finite_elements(
        &self,
        remove: Option<Blocks>,
        scale: [f64; 3],
        translate: [f64; 3],
    ) -> Result<FiniteElements, PyIntermediateError> {
        let (element_blocks, element_node_connectivity, nodal_coordinates) =
            finite_element_data_from_data(
                &self.data,
                remove,
                &Vector::new(scale),
                &Vector::new(translate),
            )?;
        Ok(FiniteElements::from_data(
            element_blocks,
            element_node_connectivity,
            nodal_coordinates.as_foo(),
        ))
    }
    /// Defeatures clusters with less than a minimum number of voxels.
    pub fn defeature(&mut self, min_num_voxels: usize) {
        self.data = defeature_voxels(
            min_num_voxels,
            super::Voxels {
                data: self.data.clone(),
            },
        )
        .get_data()
        .clone()
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
