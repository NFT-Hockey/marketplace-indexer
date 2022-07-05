use near_indexer::{InitConfigArgs, IndexerConfig, SyncModeEnum,
                   AwaitForNodeSyncedEnum, Indexer, get_default_home};
use tokio::sync::mpsc::Receiver;
use std::path::PathBuf;


fn get_home() -> PathBuf {
    // return PathBuf with current directory
    let mut current_dir = std::env::current_dir().unwrap();
    current_dir.push(".near");
    current_dir
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let home_dir = std::path::PathBuf::from(get_home());

    let command = args
        .get(1)
        .map(|arg| arg.as_str())
        .expect("You need to provide a command: `init` or `run` as arg");

    match command {
        "init" => {
            let config_args = InitConfigArgs {
                // chain / network id ()
                chain_id: Some("localnet".to_string()),
                // Account ID for validator Key
                account_id: None,
                // Specify private key generated from seed (testing only)
                test_seed: None,
                // Number of shards to initialize the chain wit
                num_shards: 1,
                // Makes block production fast
                fast: false,
                // Genesis file to use when initializing
                genesis: None,
                // Download the verified NEAR config file automatically
                download_config: false,
                // download the verififed NEAR genesis file automatically
                download_config_url: None,
                download_genesis:false,
                // specify custom download url for the genesis-file
                download_genesis_url: None,

                max_gas_burnt_view:None,

                boot_nodes:None
            };
            near_indexer::indexer_init_configs(&home_dir, config_args);
        }
        "run" => {
            let indexer_config = near_indexer::IndexerConfig {
                home_dir,
                sync_mode: near_indexer::SyncModeEnum::FromInterruption,
                await_for_node_synced: AwaitForNodeSyncedEnum::WaitForFullSync,
            };
            let sys = actix::System::new();
            sys.block_on(async move {
                let indexer = near_indexer::Indexer::new(indexer_config).unwrap();
                let stream = indexer.streamer();
                actix::spawn(listen_blocks(stream));
            });
            sys.run().unwrap();
        }
        _ => panic!("You have to pass `init` or `run` arg"),
    }
}

async fn listen_blocks(mut stream: Receiver<near_indexer::StreamerMessage>) {
    while let Some(streamer_message) = stream.recv().await {
        println!("{}", serde_json::to_value(streamer_message).unwrap());
    }
}