use std::collections::HashMap;
use tokio::runtime::Runtime;

use crate::aws;
use crate::cli;

pub fn run(_k: cli::Keez) {
    println!("export command!");

    let mut rt = Runtime::new().expect("failed to initialize runtime");
    let conf = envy_store::from_path::<HashMap<String, String>, _>("/demo");
    println!("config {:#?}", rt.block_on(conf));

    aws::parameter_store::get_parameters_by_path();

    return;
}
