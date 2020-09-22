use std::collections::HashMap;
use tokio::runtime::Runtime;

pub fn get_parameters_by_path(path_prefix: String) {
    let mut rt = Runtime::new().expect("failed to initialize runtime");
    let conf = envy_store::from_path::<HashMap<String, String>, _>(path_prefix);
    println!("config {:#?}", rt.block_on(conf));
}
