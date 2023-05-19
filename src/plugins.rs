use std::{fs::File, io::Read, process::exit, path::PathBuf, fmt::format};
use pyo3::{types::{PyDict, PyTuple, IntoPyDict}, prelude::*, exceptions};
use directories::ProjectDirs;
use serde_json::Value;
use log::{info, error, warn, debug};
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
    let mut buffer = String::from("");
    
    if let Err(err) = conf_obj.read_to_string(&mut buffer) {
        error!("Error while reading config file: {}", err);
    }
    let config: Value = serde_json::from_str(&buffer).unwrap_or_else(|err| {
        error!("Error while loading configuration values in config file: {}", err);
        exit(1);
    });
    let mut files: Vec<String>= vec![];
    if let Some(pyfiles) = config["py_files"].as_array() {
        info!("Reading python files...");
        for x in pyfiles {
            info!("Reading {}", x);
            let mut file_obj = File::open(x.as_str().unwrap()).expect("There's a non existent file in the config");
            let mut file_buffer = String::from("");
            file_obj.read_to_string(&mut file_buffer).expect("Cannot read python file");
            files.push(file_buffer);
        }
    } else {
        println!("{}", "Configuration error 1!".red());
        error!("'py_files' key in the config file doesn't exist's");
        exit(10);
    }

    let error_code = 0;

    for x in files {
        Python::with_gil(|py| {
            // Futuro api, pero por ahora no esta planeado

            let foo_module = PyModule::new(py, "RustyShellUtils").unwrap_or_else(|exc| {
                error!("Cannot create shell module. Exit Code: 20");
                exit(20);
            });

            foo_module.add_class::<RustyShellPaths>().unwrap_or_else(|_| {
                error!("Cannot create shell module. Exit Code: 30");
                exit(30);
            });
    
            // Import and get sys.modules
            let sys = PyModule::import(py, "sys").unwrap_or_else(|_| {
                error!("Cannot create shell module. Exit Code: 40");
                exit(40)
            });
            let py_modules: &PyDict = sys.getattr("modules").unwrap_or_else(|_| {
                error!("Cannot create shell module. Exit Code: 51");
                exit(51);
            }).downcast().unwrap_or_else(|_| {
                error!("Cannot create shell module. Exit Code: 52");
                exit(52);
            });
    
            // Insert foo into sys.modules
            if let Err(py_err) = py_modules.set_item("RustyShellUtils", foo_module) {
                error!("Cannot create shell module. Exit Code: 70");
                let tb = py_err.traceback(py).unwrap();
                
                let temp_dict = PyDict::new(py);
                let wrapped_tb = PyTuple::new(py, vec![tb]);

                let formated_tb = py.import("traceback").unwrap()
                .call_method("format_tb", wrapped_tb, Some(temp_dict)).unwrap()
                .extract::<String>().unwrap();
                error!("{}", formated_tb);
                info!("{}", py_err.value(py));
                exit(70);
            };
    
            // Now we can import + run our python code
            let _: Py<PyAny> = PyModule::from_code(py, &x, "", "").unwrap_or_else(|py_err| {
                error!("Cannot create shell module. Exit Code: 80");
                let tb = py_err.traceback(py).unwrap();
                
                let temp_dict = PyDict::new(py);
                let wrapped_tb = PyTuple::new(py, vec![tb]);
                let binded_locals = PyDict::new(py);
                

                let formated_tb = py.import("traceback").unwrap()
                .call_method("format_tb", wrapped_tb, Some(temp_dict)).unwrap();
                if let Err(err) = binded_locals.set_item("formated_tb", formated_tb) {
                    error!("Cannot run python code from file {}. Error: {}", x, err);
                    
                };
                let ftb = py.eval("''.join(formated_tb)", None, Some(binded_locals));

                error!("{:?}", ftb.unwrap());
                info!("{}", py_err.value(py));
                exit(80);
            }).getattr("Main").unwrap_or_else(|py_err| {
                error!("Cannot create shell module. Exit Code: 81");
                /*let tb = py_err.traceback(py).unwrap();
                
                let temp_dict = PyDict::new(py);
                let wrapped_tb = PyTuple::new(py, vec![tb]);

                let formated_tb = py.import("traceback").unwrap()
                .call_method("format_tb", wrapped_tb, Some(temp_dict)).unwrap()
                .extract::<String>().unwrap();*/
                error!("{}", py_err.get_type(py));
                error!("{}", py_err.value(py));
                exit(81);
            }).into();
        });
    };
}