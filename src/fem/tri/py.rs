use super::{
    super::{
        super::py::{IntoFoo, PyCoordinates, PyIntermediateError},
        Blocks, Connectivity, FiniteElements, Smoothing,
    },
    NUM_NODES_TRI,
};
use pyo3::prelude::*;

/// The triangular finite elements class.
#[pyclass]
pub struct TriangularFiniteElements {
    element_blocks: Blocks,
    element_node_connectivity: Connectivity<NUM_NODES_TRI>,
    nodal_coordinates: PyCoordinates,
}

#[pymethods]
impl TriangularFiniteElements {
    /// Constructs and returns a new triangular finite elements class from data.
    #[new]
    pub fn from_data(
        element_blocks: Blocks,
        element_node_connectivity: Connectivity<NUM_NODES_TRI>,
        nodal_coordinates: PyCoordinates,
    ) -> Self {
        Self {
            element_blocks,
            element_node_connectivity,
            nodal_coordinates,
        }
    }
    /// Smooths the nodal coordinates according to the provided smoothing method.
    #[pyo3(signature = (method="Taubin", hierarchical=false, iterations=10, pass_band=0.1, scale=0.6307))]
    pub fn smooth(
        &mut self,
        method: &str,
        hierarchical: bool,
        iterations: usize,
        pass_band: f64,
        scale: f64,
    ) -> Result<(), PyIntermediateError> {
        let mut finite_elements = super::TriangularFiniteElements::from_data(
            self.element_blocks.clone(),
            self.element_node_connectivity.clone(),
            self.nodal_coordinates.as_foo(),
        );
        finite_elements.node_element_connectivity()?;
        finite_elements.node_node_connectivity()?;
        if hierarchical {
            finite_elements.nodal_hierarchy()?;
        }
        finite_elements.nodal_influencers();
        match method {
            "Gauss" | "gauss" | "Gaussian" | "gaussian" | "Laplacian" | "Laplace" | "laplacian"
            | "laplace" => {
                finite_elements.smooth(Smoothing::Laplacian(iterations, scale))?;
            }
            "Taubin" | "taubin" => {
                finite_elements.smooth(Smoothing::Taubin(iterations, pass_band, scale))?;
            }
            _ => return Err(format!("Invalid smoothing method {} specified.", method))?,
        }
        self.element_blocks = finite_elements.element_blocks;
        self.element_node_connectivity = finite_elements.element_node_connectivity;
        self.nodal_coordinates = finite_elements.nodal_coordinates.as_foo();
        Ok(())
    }
}
