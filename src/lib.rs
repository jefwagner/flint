#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

pub use crate::flint::Flint;

mod utils;
mod flint;
mod comp;
mod ops;
mod math;

#[cfg(feature = "pyo3")]
mod pyobj;

#[cfg(feature = "pyo3")]
use crate::pyobj::PyFlint;

#[cfg(feature = "pyo3")]
#[pymodule]
pub fn flint_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyFlint>()?;
    Ok(())
}