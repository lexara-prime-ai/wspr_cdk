#![allow(unused)]
// Import [python] types.
use pyo3::types::PyDict;
use pyo3::{prelude::*, wrap_pyfunction};
use wspr_cdk::{services::prelude::*, state::prelude::*};

// Wrap ClickHouseState in a new struct that implements IntoPy.
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
        The '__repr__'  method will provide a meaningful [string] representation
        for the ClickHouseState, python <object>.

        ...........................................................................

        The '__str__'  method will provide a user-friendly [string] representation
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

    fn get_data(&self) -> Vec<PyObject> {
        Python::with_gil(|py| {
            self.inner
                .DATA
                .as_ref()
                .map(|data| {
                    data.iter()
                        .map(|spot| {
                            let dict = PyDict::new(py);
                            dict.set_item("id", spot.id).unwrap();
                            dict.set_item("time", spot.time.to_string()).unwrap();
                            dict.set_item("band", spot.band).unwrap();
                            dict.set_item("rx_sign", spot.rx_sign.clone()).unwrap();
                            dict.set_item("rx_lat", spot.rx_lat).unwrap();
                            dict.set_item("rx_lon", spot.rx_lon).unwrap();
                            dict.set_item("rx_loc", spot.rx_loc.clone()).unwrap();
                            dict.set_item("tx_sign", spot.tx_sign.clone()).unwrap();
                            dict.set_item("tx_lat", spot.tx_lat).unwrap();
                            dict.set_item("tx_lon", spot.tx_lon).unwrap();
                            dict.set_item("tx_loc", spot.tx_loc.clone()).unwrap();
                            dict.set_item("distance", spot.distance).unwrap();
                            dict.set_item("azimuth", spot.azimuth).unwrap();
                            dict.set_item("rx_azimuth", spot.rx_azimuth).unwrap();
                            dict.set_item("frequency", spot.frequency).unwrap();
                            dict.set_item("power", spot.power).unwrap();
                            dict.set_item("snr", spot.snr).unwrap();
                            dict.set_item("drift", spot.drift).unwrap();
                            dict.set_item("version", spot.version.clone()).unwrap();
                            dict.set_item("code", spot.code).unwrap();
                            dict.into()
                        })
                        .collect()
                })
                .unwrap_or_else(Vec::new)
        })
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
fn python_wrapper(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ClickHouseStateWrapper>()?;
    m.add_function(wrap_pyfunction!(get_wspr_spots, m)?)?;
    Ok(())
}
