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
    let raw_parameters = raw_parameters_by_path(path_prefix.clone());
    let result = Parameters::new(path_prefix.clone());
    return result;
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

    println!("{:?}", params);

    return Ok(params);
}
