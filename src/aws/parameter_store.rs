use rusoto_ssm::{GetParametersByPathRequest, Ssm, SsmClient};
use std::collections::HashMap;
use std::error;
use tokio::runtime;

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

fn raw_parameters_by_path(
    path_prefix: String,
) -> Result<Vec<rusoto_ssm::Parameter>, Box<dyn error::Error>> {
    println!("in raw_parameters_by_path...");

    let mut rt = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    let client = SsmClient::new(Default::default());
    let res = rt.block_on(client.get_parameters_by_path(GetParametersByPathRequest {
        path: path_prefix.clone(),
        with_decryption: Some(true),
        recursive: Some(true),
        ..GetParametersByPathRequest::default()
    }));

    println!("{:?}", res);
    let mut vec = Vec::new();

    return Ok(vec);
}
