use pyo3::prelude::*;
use pyo3::types::IntoPyDict;
use std::fs::File;
use std::io::{Write};

#[pyclass]
struct FileWrapper {
    file: File,
}

#[pymethods]
impl FileWrapper {
    #[new]
    #[args(file_path)]
    fn new(file_path: &str) -> PyResult<FileWrapper> {
        if let Err(err) = std::fs::create_dir_all(file_path) {
            Err(PyErr::new::<exceptions::IOError, _>("Error while trying to create unexisting directories"))
        };
        match File::create(file_path) {
            Ok(file) => Ok(FileWrapper { file }),
            Err(err) => Err(PyErr::new::<exceptions::IOError, _>(err.to_string())),
        }
    }

    #[text_signature = "(self, text)"]
    fn write(&mut self, text: &str) -> PyResult<()> {
        match self.file.write_all(text.as_bytes()) {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<exceptions::IOError, _>(err.to_string())),
        }
    }

    #[text_signature = "(self)"]
    fn flush(&mut self) -> PyResult<()> {
        match self.file.flush() {
            Ok(_) => Ok(()),
            Err(err) => Err(PyErr::new::<exceptions::IOError, _>(err.to_string())),
        }
    }
}
