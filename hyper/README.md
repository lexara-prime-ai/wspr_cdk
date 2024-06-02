# WSPR CDK Python Wrapper Documentation

This document provides information on how to use the `wspr_cdk` Python wrapper implemented in Rust.

## Rust Code Implementation

The following Rust code defines the `ClickHouseStateWrapper` struct and exposes it to Python using the **PyO3** library. It includes methods to initialize the _state_, and it provides a meaningful string representation of the _internal state_.

### Dependencies

Add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
pyo3 = { version = "0.17", features = ["extension-module"] }
pyo3-asyncio = { version = "0.17", features = ["tokio-runtime"] }
tokio = { version = "1", features = ["full"] }
wspr_cdk = { path = "../wspr_cdk" }  # Adjust the path as needed
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

    // Add more methods to interact with ClickHouseState if needed.
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

if __name__ == '__main__':
    asyncio.run(main())
```

### Explanation

- **Import the module**: Import the generated Python module `python_wrapper.python_wrapper`.
- **Define the async function**: Define an asynchronous function `main()` that calls `get_wspr_spots` with the parameters `"10"` and `"JSON"`.
- **Print the output**: The result of `get_wspr_spots` is printed, showing the internal state of the `ClickHouseStateWrapper`.

### Expected Output

The output will show the string representation of the `ClickHouseStateWrapper` object:

```sh
Output: ClickHouseStateWrapper with some internal state: <actual state details>
```

`ClickHouseState` implements the `Debug` trait to provide detailed information when formatted with `{:?}`.

### To do

- Adjust the `__repr__` and `__str__` methods to customize the output as needed.
