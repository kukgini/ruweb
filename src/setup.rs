
async fn download_genesis_file() -> std::result::Result<String, String> {
    match std::env::var("GENESIS_FILE").ok() {
        Some(genesis_file) => {
            if !std::path::Path::new(&genesis_file).exists() {
                Err(format!("The file {} does not exist", genesis_file))
            } else {
                log::info!("Using genesis file {}", genesis_file);
                Ok(genesis_file)
            }
        }
        None => {
            Err(format!("environment variable GENESIS_FILE is not set"))
        }
    }
}

pub async fn initialize() {
    let genesis_path = download_genesis_file()
        .await
        .expect("Failed to download the genesis file");
}