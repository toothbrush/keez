use std::collections::HashMap;

#[derive(Debug)]
pub struct Parameter {
    parameter_value: String,
    parameter_type: String,
    modified: bool,
}

#[derive(Debug)]
pub struct Parameters {
    prefix: String,
    ps: HashMap<String, Parameter>,
}

impl Parameters {
    pub fn new(prefix: String) -> Parameters {
        return Parameters {
            prefix: String::from(prefix),
            ps: HashMap::new(),
        };
    }
}

pub fn get_parameters_by_path(path_prefix: String) -> Parameters {
    let ps = raw_parameters_by_path(path_prefix.clone());
    return Parameters::new(path_prefix.clone());
}

fn raw_parameters_by_path(path_prefix: String) -> Vec<rusoto_ssm::Parameter> {
    let mut vec = Vec::new();

    println!("in raw_parameters_by_path...");

    return vec;
}
