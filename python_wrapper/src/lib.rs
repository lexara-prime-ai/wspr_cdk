#![allow(unused)]
use pyo3::{prelude::*, wrap_pyfunction};
use wspr_cdk::{services::prelude::*, state::prelude::*};

/// Wrap ClickHouseState in a new struct that implements IntoPy.
#[pyclass]
struct ClickHouseStateWrapper {
    inner: ClickHouseState,
}

#[pymethods]
impl ClickHouseStateWrapper {
    #[new]
    fn new() -> Self {
        ClickHouseStateWrapper {
            inner: ClickHouseClient::init(),
        }
    }

    /*
        >> The '__repr__'  method will provide a meaningful [string] representaion
        for the ClickHouseState, python <object>.
        _________________________________________________________________
        >> The '__str__'  method will provide a user-friendly [string] representaion
        for the ClickHouseState, python <object>.
    */
    fn __repr__(&self) -> String {
        format!(
            "ClickHouseStateWrapper with some internal state: {:?}",
            self.inner
        )
    }

    fn __str__(&self) -> String {
        format!(
            "ClickHouseStateWrapper with some internal state: {:?}",
            self.inner
        )
    }

    // [To Do] -> Add more methods to interact with ClickHouseState if needed.
}

#[pyfunction]
fn get_wspr_spots(py: Python, limit: String, result_format: String) -> PyResult<&PyAny> {
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    pyo3_asyncio::tokio::future_into_py(py, async move {
        ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, &limit, &result_format).await;

        Ok(Python::with_gil(|py| {
            // Convert the state into a Python <object>.
            Py::new(py, ClickHouseStateWrapper { inner: state }).unwrap()
        }))
    })
}

/// The <wspr_cdk> python module, implemented in Rust.
#[pymodule]
fn python_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_wspr_spots, m)?)?;
    Ok(())
}