use pyo3::prelude::*;

#[pyfunction]
pub fn tls(
) {
    _ = native_tls::TlsConnector::new();
}

#[pymodule]
fn issue(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(tls))?;
    Ok(())
}
