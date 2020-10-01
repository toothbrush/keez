use std::collections::HashMap;
use std::error;
use std::fmt;
use std::str::FromStr;

use regex::Regex;
use rusoto_ssm::{GetParametersByPathRequest, PutParameterRequest, Ssm, SsmClient};
use serde::{Deserialize, Serialize};
use tokio::runtime;

use crate::flags::operation_mode::OperationMode;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(rename = "value")]
    parameter_value: String,
    #[serde(rename = "type")]
    parameter_type: ParameterType,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum ParameterType {
    String,
    SecureString,
    StringList,
}

#[derive(Debug)]
pub enum ParameterError {
    InvalidParameterType(
        /// Contains the malformed input for debugging purposes
        String,
    ),
    InvalidPathPrefix(
        /// Contains a more detailed error description
        String,
    ),
    NonexistentKey(String),
}

// TODO i'm sure this can be made less ugly.
impl fmt::Display for ParameterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ParameterType::String => "String",
                ParameterType::SecureString => "SecureString",
                ParameterType::StringList => "StringList",
            }
        )
    }
}

impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParameterError::InvalidParameterType(input) => {
                write!(f, "invalid AWS Parameter Store parameter type: {:?}", input)
            }
            ParameterError::InvalidPathPrefix(desc) => write!(f, "invalid path prefix: {}", desc),
            ParameterError::NonexistentKey(desc) => write!(f, "key {} not found in list of parameters.  Use `create` command to add new parameters.", desc),
        }
    }
}

impl error::Error for ParameterError {}

impl FromStr for ParameterType {
    type Err = ParameterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "String" => Ok(ParameterType::String),
            "SecureString" => Ok(ParameterType::SecureString),
            "StringList" => Ok(ParameterType::StringList),
            _ => Err(ParameterError::InvalidParameterType(s.to_string()).into()),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParameterCollection {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    prefix: String,
    parameters: HashMap<String, Parameter>,
}

impl Parameter {
    pub fn new(parameter_value: String, parameter_type: String) -> Parameter {
        return Parameter {
            parameter_value,
            parameter_type: ParameterType::from_str(&parameter_type).unwrap(),
        };
    }
}

impl ParameterCollection {
    pub fn new(prefix: String) -> ParameterCollection {
        return ParameterCollection {
            prefix,
            parameters: HashMap::new(),
        };
    }

    pub fn parameters(&self) -> &HashMap<String, Parameter> {
        &self.parameters
    }

    pub fn prefix(&self) -> &String {
        &self.prefix
    }
}

pub fn get_parameters_by_path(
    path_prefix: String,
    debug: bool,
) -> Result<ParameterCollection, Box<dyn error::Error>> {
    let raw_parameters = raw_parameters_by_path(path_prefix.clone(), debug)?;
    let mut result = ParameterCollection::new(path_prefix.clone());

    for raw_param in &raw_parameters {
        result.parameters.insert(
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
    debug: bool,
) -> Result<Vec<rusoto_ssm::Parameter>, Box<dyn error::Error>> {
    let mut rt = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?;

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

    let mut parameters: Vec<rusoto_ssm::Parameter> = Vec::new();
    if let Some(new_params) = res.parameters {
        parameters.extend(new_params.into_iter());
    }

    // Get next set of parameters if there's a next_token.
    while let Some(next_token) = res.next_token {
        req.next_token = Some(next_token);
        res = rt.block_on(client.get_parameters_by_path(req.clone()))?;

        if let Some(new_params) = res.parameters {
            parameters.extend(new_params.into_iter());
        }
    }

    if debug {
        eprintln!("raw_parameters_by_path: received from API:");
        eprintln!("{:?}", parameters);
    }

    return Ok(parameters);
}

fn check_path(parameter_path: String) -> Result<(), Box<dyn error::Error>> {
    let re = Regex::new(r"^/[a-zA-Z0-9_.-]").unwrap();
    if !(re.is_match(&parameter_path)) {
        return Err(ParameterError::InvalidPathPrefix("must begin with slash".to_string()).into());
    }

    if parameter_path.ends_with("/") {
        return Err(ParameterError::InvalidPathPrefix(
            "must not have a trailing slash".to_string(),
        )
        .into());
    }

    Ok(())
}

/// reroot_parameters takes a set of parameters and target prefix and
/// changes the source to the target prefix.  typically it's useful as
/// an input to `push_new_parameters`.
pub fn reroot_parameters(
    source: ParameterCollection,
    destination: String,
) -> Result<ParameterCollection, Box<dyn error::Error>> {
    check_path(destination.clone())?;

    // Build a regex with the old parameter path prefix which we wish
    // to replace.

    let mut new_params: HashMap<String, Parameter> = HashMap::new();
    let mut new_key: String;

    for (key, param) in source.parameters() {
        // It's okay to panic here, because things are weird if the
        // search prefix doesn't match all the keys in a blob.
        let mut new_key_parts = Vec::new();
        new_key_parts.push(destination.clone());
        new_key_parts.push(
            key.clone()
                .strip_prefix(source.prefix())
                .unwrap()
                .to_string(),
        );
        new_key = new_key_parts.join("");
        new_params.insert(
            new_key,
            Parameter {
                parameter_value: param.parameter_value.clone(),
                parameter_type: param.parameter_type.clone(),
            },
        );
    }

    return Ok(ParameterCollection {
        prefix: destination,
        parameters: new_params,
    });
}

pub fn push_new_parameters(
    parameters: ParameterCollection,
    operation_mode: OperationMode,
) -> Result<(), Box<dyn error::Error>> {
    let mut rt = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?;

    let client = SsmClient::new(Default::default());

    let mut req: PutParameterRequest;

    for (key, param) in parameters.parameters() {
        if operation_mode == OperationMode::ReadWrite {
            eprintln!("Creating key {}...", key);
            req = PutParameterRequest {
                data_type: Some("text".to_string()),
                name: key.to_string(),
                type_: Some(param.parameter_type.to_string()),
                value: param.parameter_value.clone(),
                overwrite: Some(false),
                ..PutParameterRequest::default()
            };

            // TODO catch specific ParameterAlreadyExists error,
            // because that's a user's fault.
            rt.block_on(client.put_parameter(req.clone()))?;
        } else {
            eprintln!("[DRY-RUN] Would create key {}...", key);
        }
    }
    Ok(())
}

// push_updated_parameters should be called after interactively
// modifying a set of parameters.  We also want the original
// parameters so that we can do a comparison.  We don't want the user
// sneaking in new parameters or making a typo and saving /foo to
// /fooprime accidentally.
pub fn push_updated_parameters(
    old_parameters: ParameterCollection,
    new_parameters: ParameterCollection,
    operation_mode: OperationMode,
) -> Result<(), Box<dyn error::Error>> {
    let mut updated_parameters: HashMap<String, Parameter> = HashMap::new();

    for (key, new_param) in new_parameters.parameters() {
        match old_parameters.parameters().get(key) {
            Some(old_param) => {
                // alright, the key exists in the old hashmap
                if old_param.parameter_type != new_param.parameter_type
                    || old_param.parameter_value != new_param.parameter_value
                {
                    // okay, something has changed.
                    updated_parameters.insert(
                        key.clone(),
                        Parameter {
                            parameter_type: new_param.parameter_type.clone(),
                            parameter_value: new_param.parameter_value.clone(),
                        },
                    );
                }
            }
            None => return Err(ParameterError::NonexistentKey(key.to_string()).into()),
        }
    }

    let mut rt = runtime::Builder::new()
        .threaded_scheduler()
        .enable_all()
        .build()?;

    let client = SsmClient::new(Default::default());

    let mut req: PutParameterRequest;

    for (key, param) in updated_parameters {
        if operation_mode == OperationMode::ReadWrite {
            eprintln!("Updating key {}...", key);
            req = PutParameterRequest {
                data_type: Some("text".to_string()),
                name: key,
                type_: Some(param.parameter_type.to_string()),
                value: param.parameter_value,
                overwrite: Some(true),
                ..PutParameterRequest::default()
            };

            rt.block_on(client.put_parameter(req.clone()))?;
        } else {
            eprintln!("[DRY-RUN] Would update key {}...", key);
        }
    }
    Ok(())
}
