// Relative Modules
pub mod python;

// Standard Uses

// Crate Uses

// External Uses


#[allow(unused)]
pub fn init() {
    let python_interp = python::create_interpreter().unwrap();
}

