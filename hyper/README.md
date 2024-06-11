# WSPR CDK Python Wrapper Documentation

This document provides information on how to use the `wspr_cdk` Python wrapper implemented in Rust.

## Rust Code Implementation

The following Rust code defines the `ClickHouseStateWrapper` struct and exposes it to Python using the **PyO3** library. It includes methods to initialize the _state_, provides a meaningful string representation of the _internal state_, and includes a method to return the data in a Python-friendly format.

### Dependencies

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.17", features = ["extension-module"] }
pyo3-asyncio = { version = "0.17", features = ["tokio-runtime"] }
tokio = { version = "1", features = ["full"] }
wspr_cdk = { path = "../wspr_cdk" }  # Adjust the path as needed.
```

### Rust Code

```rust
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

    /// The '__repr__' method provides a meaningful string representation for the ClickHouseState Python object.
    fn __repr__(&self) -> String {
        format!(
            "ClickHouseStateWrapper with some internal state: {:?}",
            self.inner
        )
    }

    /// The '__str__' method provides a user-friendly string representation for the ClickHouseState Python object.
    fn __str__(&self) -> String {
        format!(
            "ClickHouseStateWrapper with some internal state: {:?}",
            self.inner
        )
    }

    /// The `get_data` method returns the internal data in a Python-friendly format.
    fn get_data(&self) -> Vec<PyObject> {
        Python::with_gil(|py| {
            self.inner
                .data
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
}

#[pyfunction]
fn get_wspr_spots(py: Python, limit: String, result_format: String) -> PyResult<&PyAny> {
    let mut state = ClickHouseClient::init();
    let session = session_manager::SessionManager::new();

    pyo3_asyncio::tokio::future_into_py(py, async move {
        ClickHouseClient::dispatch(&mut state, ClickHouseAction::Get, &limit, &result_format).await;

        Ok(Python::with_gil(|py| {
            // Convert the state into a Python object.
            Py::new(py, ClickHouseStateWrapper { inner: state }).unwrap()
        }))
    })
}

/// The wspr_cdk Python module, implemented in Rust.
#[pymodule]
fn python_wrapper(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_wspr_spots, m)?)?;
    Ok(())
}
```

## Usage in Python

The following Python script demonstrates how to use the `python_wrapper` module to fetch **_WSPR spots_**.

### Installation

Use `maturin` to build and install the Rust module as a Python package.

1.  Install `maturin`: `pip install maturin`
2.  Build and develop the package: `maturin develop`

### Python Script

```python
import asyncio
import python_wrapper.python_wrapper

async def main():
    output = await python_wrapper.python_wrapper.get_wspr_spots("10", "JSON")
    print('Output: ', output)

    # Get the data in Python-friendly format
    data = output.get_data()
    print('Data: ', data)

if __name__ == '__main__':
    asyncio.run(main())
```

### Explanation

- **Import the module**: Import the generated Python module `python_wrapper.python_wrapper`.
- **Define the async function**: Define an asynchronous function `main()` that calls `get_wspr_spots` with the parameters `"10"` and `"JSON"`.
- **Print the output**: The result of `get_wspr_spots` is printed, showing the internal state of the `ClickHouseStateWrapper`.
- **Get the data**: Use the `get_data` method to retrieve the data in a Python-friendly format and print it.

### Expected Output

The output will show the string representation of the `ClickHouseStateWrapper` object and the data in a Python-friendly format:

```sh
Output: ClickHouseStateWrapper with some internal state: <actual state details>
Output: [{'id': 7791848067, 'time': '2024-06-03T04:34:00', 'band': 0, ...}, ...]
```

`ClickHouseState` implements the `Debug` trait to provide detailed information when formatted with `{:?}`.

### To do

- Adjust the `__repr__` and `__str__` methods to customize the output as needed.
