// Relative Modules
pub(crate) mod html;

// Standard Uses

// Crate Uses

// External Uses
use pyo3::prelude::*;
use pyo3::types::IntoPyDict;


pub fn create_interpreter() -> PyResult<()> {
    let interp = Python::with_gil(|py| {
        let sys = py.import("sys")?;
        let version: String = sys.getattr("version")?.extract()?;

        let locals = [("os", py.import("os")?)].into_py_dict(py);
        let code = "os.getenv('USER') or os.getenv('USERNAME') or 'Unknown'";
        let user: String = py.eval(code, None, Some(&locals))?.extract()?;

        println!("Hello {}, I'm Python {}", user, version);
        Ok(())
    });

    interp
}

