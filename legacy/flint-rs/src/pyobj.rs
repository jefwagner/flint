use std::ops::{
    Neg,
    Add,
    Sub,
    Mul,
    Div,
};
use pyo3::{
    prelude::*,
    exceptions::PyTypeError,
    class::basic::CompareOp,
};

use crate::flint::Flint;

/// The python object version of the flint object
#[pyclass(name="Flint")]
#[derive(Debug, Clone, Copy)]
pub struct PyFlint{
    pub f: Flint
}


/// Convert from PyAny to base Flint
fn ob_to_flint<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Flint> {
    if let Ok(flint) = ob.downcast::<PyFlint>() {
        let a = flint.borrow().f.a;
        let b = flint.borrow().f.b;
        Ok(Flint { a, b })
    } else if let Ok((a, b)) = ob.extract::<(f64, f64)>() {
        Ok(Flint { a, b })
    } else if let Ok(i) = ob.extract::<i32>() {
        Ok(Flint::from(i))
    } else if let Ok(i) = ob.extract::<i64>() {
        Ok(Flint::from(i))
    } else if let Ok(i) = ob.extract::<i128>() {
        Ok(Flint::from(i))
    } else if let Ok(f) = ob.extract::<f64>() {
        Ok(Flint::from(f))
    } else {
        Err(PyTypeError::new_err("Could not convert into flint."))
    }
}

#[pymethods]
impl PyFlint {

    /// The python constructor
    #[new]
    fn py_new<'py>(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(PyFlint{ f: ob_to_flint(ob)? })
    }

    /// Get the lower boundary of the interval
    #[getter]
    fn a(&self) -> f64 {
        self.f.a
    }

    /// Get the upper boundary of the interval
    #[getter]
    fn b(&self) -> f64 {
        self.f.b
    }

    /// The debug print statement
    fn __str__(&self) -> String {
        format!("{:?}", self.f)
    }

    /// The formated print statement
    fn __repr__(&self) -> String {
        format!("{}", self.f)
    }

    /// Copy the midpoint and width methods from rust object
    fn midpoint(&self) -> f64 { self.f.midpoint() }
    fn width(&self) -> f64 { self.f.midpoint() }

    /// implement the richcompare using comparison operators from rust implementation
    fn __richcmp__<'py>(&self, other: &Bound<'py, PyAny>, op: CompareOp) -> PyResult<bool> {
        let other = ob_to_flint(other)?;
        match op {
            CompareOp::Lt => Ok(self.f < other),
            CompareOp::Le => Ok(self.f <= other),
            CompareOp::Eq => Ok(self.f == other),
            CompareOp::Ne => Ok(self.f != other),
            CompareOp::Gt => Ok(self.f > other),
            CompareOp::Ge => Ok(self.f >= other),
        }
    }

    /// implement arithmetic operators
    fn __neg__(&self) -> Self {
        PyFlint{ f: self.f.neg() }
    }

    fn __add__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.add(other) })
    }

    fn __radd__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: other.add(self.f) })
    }

    fn __sub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.sub(other) })
    }

    fn __rsub__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: other.sub(self.f) })
    }

    fn __mul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.mul(other) })
    }

    fn __rmul__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: other.mul(self.f) })
    }

    fn __truediv__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.div(other) })
    }

    fn __rtruediv__<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: other.div(self.f) })
    }

    /// single variable math functions
    fn is_nan(&self) -> bool { self.f.is_nan() }
    fn is_infinite(&self) -> bool { self.f.is_infinite() }
    fn is_finite(&self) -> bool { self.f.is_finite() }
    fn nonzero(&self) -> bool { self.f.nonzero() }
    fn abs(&self) -> PyFlint { PyFlint{ f: self.f.abs() }}
    fn sqrt(&self) -> PyFlint { PyFlint{ f: self.f.sqrt() }}
    fn cbrt(&self) -> PyFlint { PyFlint{ f: self.f.cbrt() }}
    fn exp(&self) -> PyFlint { PyFlint{ f: self.f.exp() }}
    fn exp2(&self) -> PyFlint { PyFlint{ f: self.f.exp2() }}
    fn exp_m1(&self) -> PyFlint { PyFlint{ f: self.f.exp_m1() }}
    fn ln(&self) -> PyFlint { PyFlint{ f: self.f.ln() }}
    fn log10(&self) -> PyFlint { PyFlint{ f: self.f.log10() }}
    fn log2(&self) -> PyFlint { PyFlint{ f: self.f.log2() }}
    fn ln_1p(&self) -> PyFlint { PyFlint{ f: self.f.ln_1p() }}
    fn sin(&self) -> PyFlint { PyFlint{ f: self.f.sin() }}
    fn cos(&self) -> PyFlint { PyFlint{ f: self.f.cos() }}
    fn tan(&self) -> PyFlint { PyFlint{ f: self.f.tan() }}
    fn asin(&self) -> PyFlint { PyFlint{ f: self.f.asin() }}
    fn acos(&self) -> PyFlint { PyFlint{ f: self.f.acos() }}
    fn atan(&self) -> PyFlint { PyFlint{ f: self.f.atan() }}
    fn sinh(&self) -> PyFlint { PyFlint{ f: self.f.sinh() }}
    fn cosh(&self) -> PyFlint { PyFlint{ f: self.f.cosh() }}
    fn tanh(&self) -> PyFlint { PyFlint{ f: self.f.tanh() }}
    fn asinh(&self) -> PyFlint { PyFlint{ f: self.f.asinh() }}
    fn acosh(&self) -> PyFlint { PyFlint{ f: self.f.acosh() }}
    fn atanh(&self) -> PyFlint { PyFlint{ f: self.f.atanh() }}

    /// multi-variable math functions
    fn powf<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.powf(other) })
    }
    fn log<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.log(other) })
    }
    fn hypot<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.hypot(other) })
    }
    fn atan2<'py>(&self, other: &Bound<'py, PyAny>) -> PyResult<PyFlint> {
        let other = ob_to_flint(other)?;
        Ok(PyFlint{ f: self.f.atan2(other) })
    }

}
