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
pub struct ParameterCollection {
    prefix: String,
    params: HashMap<String, Parameter>,
}

impl Parameter {
    pub fn new(parameter_value: String, parameter_type: String) -> Parameter {
        return Parameter {
            parameter_value,
            parameter_type,
            modified: false,
        };
    }
}

impl ParameterCollection {
    pub fn new(prefix: String) -> ParameterCollection {
        return ParameterCollection {
            prefix,
            params: HashMap::new(),
        };
    }
}

pub fn get_parameters_by_path(
    path_prefix: String,
) -> Result<ParameterCollection, Box<dyn error::Error>> {
    let raw_parameters = raw_parameters_by_path(path_prefix.clone())?;
    let mut result = ParameterCollection::new(path_prefix.clone());

    for raw_param in &raw_parameters {
        result.params.insert(
            raw_param.name.clone().unwrap(), // TODO clone or borrow??
            Parameter::new(
                raw_param.value.clone().unwrap_or_default(),
                raw_param.type_.clone().unwrap_or_default(),
            ),
        );
    }

    return Ok(result);
}

fn raw_parameters_by_path(
    path_prefix: String,
) -> Result<Vec<rusoto_ssm::Parameter>, Box<dyn error::Error>> {
    let mut rt = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()
        .unwrap();

    let client = SsmClient::new(Default::default());

    // Super awful, let's chuck out all the lovingly-crafted futures
    // work and just run the request in a blocking fashion on the main
    // thread.
    let mut req = GetParametersByPathRequest {
        path: path_prefix.to_owned(),
        with_decryption: Some(true),
        recursive: Some(true),
        ..GetParametersByPathRequest::default()
    };

    let mut res = rt.block_on(client.get_parameters_by_path(req.clone()))?;

    let mut params: Vec<rusoto_ssm::Parameter> = Vec::new();
    if let Some(new_params) = res.parameters {
        params.extend(new_params.into_iter());
    }

    // Get next set of parameters if there's a next_token.
    while let Some(next_token) = res.next_token {
        req.next_token = Some(next_token);
        res = rt.block_on(client.get_parameters_by_path(req.clone()))?;

        if let Some(new_params) = res.parameters {
            params.extend(new_params.into_iter());
        }
    }

    let debug = true; // TODO filter down from --debug structopt
    if debug {
        println!("raw_parameters_by_path: received from API:");
        println!("{:?}", params);
    }

    return Ok(params);
}
