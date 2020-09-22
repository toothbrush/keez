pub mod cmd_export {

    use std::collections::HashMap;
    use tokio::runtime::Runtime;

    use crate::Keez;

    pub fn run(_k: Keez) {
        println!("export command!");

        let mut rt = Runtime::new().expect("failed to initialize runtime");
        let conf = envy_store::from_path::<HashMap<String, String>, _>("/demo");
        println!("config {:#?}", rt.block_on(conf));
        return;
    }
}
