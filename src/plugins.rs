mod stdout;

use std::{fs::File, io::Read, path::PathBuf};
use pyo3::{types::{PyDict}, prelude::*, exceptions};
use directories::ProjectDirs;
use serde_json::Value;
use log::{info, error, debug};
use colored::*;


#[pyclass]
pub struct RustyShellPaths {
    data_path: String,
    configs_path: String
}


#[pymethods]
impl RustyShellPaths {
    #[new]
    pub fn new(plugin_name: String) -> PyResult<Self> {
        if let Some(conf_obj) = ProjectDirs::from("", "", "RustyShell") {
            let conf_path = conf_obj.config_dir();
            let data_path = conf_obj.data_dir();
            let mut individual_conf_path = PathBuf::new();
            individual_conf_path.push(conf_path);
            individual_conf_path.push(plugin_name);
            if let Err(err_dir) = std::fs::create_dir_all(individual_conf_path.clone()) {
                println!("{}: Error al crear la carpeta. Informacion: {}", "Fatal error".red(), err_dir);
                return Err(PyErr::new::<exceptions::PyIOError, _>("Error al crear la carpeta"));
            }
            return Ok(RustyShellPaths{ data_path:data_path.to_string_lossy().into_owned(), configs_path:individual_conf_path.to_string_lossy().into_owned() });
        } else {
            return Err(PyErr::new::<exceptions::PyValueError, _>("Error al obtener la carpeta de configuracion"));
        };
    }

    #[getter]
    pub fn conf_path(&self) -> String {
        self.configs_path.clone()
    }

    #[getter]
    pub fn data_dir(&self) -> String {
        self.data_path.clone()
    }
}


pub fn load_python_plugin_init_files() {
    info!("Opening configuration...");
    let mut conf_obj = File::open("config/preferences.json").expect("El archivo de configuracion no existe");
    let mut buffer = String::new();
    
    if let Err(err) = conf_obj.read_to_string(&mut buffer) {
        error!("Error while reading config file: {}", err);
        return;
    }
    let config: Value = match serde_json::from_str(&buffer) {
        Ok(val) => val,
        Err(err) => {
            error!("Error while loading configuration values in config file: {}", err);
            return;
        }
    };
    let mut files: Vec<String>= vec![];
    if let Some(pyfiles) = config["py_files"].as_array() {
        info!("Reading python files...");
        for x in pyfiles {
            info!("Reading {}", x);
            if let Ok(mut file_obj) = File::open(x.as_str().unwrap()) {
                let mut file_buffer = String::new();
                if let Err(err) = file_obj.read_to_string(&mut file_buffer) {
                    error!("Cannot read python file: {}", err);
                    continue;
                }
                files.push(file_buffer);
            } else {
                println!("{}", "Configuration error 1!".red());
                error!("There's a non-existent file in the config: {}", x);
            }
        }
    } else {
        println!("{}", "Configuration error 1!".red());
        error!("'py_files' key in the config file doesn't exist's");
        return;
    }

    for x in files {
        Python::with_gil(|py| {
            // Futuro api, pero por ahora no esta planeado

            let foo_module = match PyModule::new(py, "RustyShellUtils") {
                Ok(module) => module,
                Err(exc) => {
                    error!("Cannot create shell module. Exit Code: 20");
                    debug!("Error ocurred while trying to create the module");
                    error!("{}", exc.value(py));
                    return;
                }
            };

            if let Err(exc) = foo_module.add_class::<RustyShellPaths>() {
                error!("Cannot create shell module. Exit Code: 30");
                debug!("Error ocurred while trying to add class to the module");
                error!("{}", exc.value(py));
                return;
            }
    
            // Import and get sys.modules
            let sys = match PyModule::import(py, "sys") {
                Ok(module) => module,
                Err(exc) => {
                    error!("Cannot create shell module. Exit Code: 40");
                    debug!("Error ocurred while trying to import sys module");
                    error!("{}", exc.value(py));
                    return;
                }
            };
            let py_modules: &PyDict = match sys.getattr("modules") {
                Ok(modules) => modules.downcast::<PyDict>().unwrap(),
                Err(_) => {
                    error!("Cannot create shell module. Exit Code: 51");
                    return;
                }
            };
            sys.setattr(attr_name, value)
    
            // Insert foo into sys.modules
            if let Err(py_err) = py_modules.set_item("RustyShellUtils", foo_module) {
                error!("Cannot create shell module. Exit Code: 70");
                debug!("Error while trying to execute py_modules.set_item()");
                info!("{}", py_err.value(py));
                return;
            };
            
            

            // Now we can import + run our python code
            match PyModule::from_code(py, &x, "", "") {
                Ok(module) => {
                    if let Err(py_err) = module.getattr("Main") {
                        error!("Cannot create shell module. Exit Code: 81");
                        error!("Main class not found");
                        error!("{}", py_err.get_type(py));
                        error!("{}", py_err.value(py));
                        return;
                    }
                }
                Err(py_err) => {
                    error!("Cannot create shell module. Exit Code: 80");
                    error!("Python related error");
                    error!("{}", py_err.get_type(py));
                    error!("{}", py_err.value(py));
                    return;
                }
            }
        });
    };
}