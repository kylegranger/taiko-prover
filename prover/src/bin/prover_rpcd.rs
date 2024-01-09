use clap::Parser;
use thiserror::Error;
// use env_logger::Env;

// use prover::server::serve;
// use prover::shared_state::SharedState;
use prover::VERSION;

#[derive(Error, Debug)]
pub enum GevulotError {
    #[error("Error with base64 decoding")]
    Base64DecodeError,
    #[error("Error with base64 encoding")]
    Base64EncodeError,
    #[error("Error with canonical serialization")]
    CanonicalDeserializeError,
    #[error("Error with canonical deserialization")]
    CanonicalSerializeError,
    #[error("IO Error")]
    ErrorIo,
    #[error("Error with JSON serialization")]
    JsonDeserializeError,
    #[error("Error with JSON deserialization")]
    JsonSerializeError,
    #[error("Could not parse the r1cs file data")]
    R1csParseError,
    #[error("Could not parse the r1cs file data")]
    WtnsParseError,
    #[error("Error in Groth16 setup")]
    Groth16SetupError,
    #[error("Error in Groth16 verification")]
    Groth16VerifyError,
    #[error("Error in Marlin verification")]
    MarlinVerifyError,
    #[error("Filecoin window post error")]
    FilecoinWindowPostError,
    #[error("Filecoin winning post error")]
    FilecoinWinningPostError,
}

use eth_types::{Block, Transaction};

#[derive(Parser, Debug)]
#[clap(version = VERSION, about)]
/// This command starts a http/json-rpc server and serves proof oriented methods.
pub(crate) struct ProverdConfig {
    #[clap(long, env = "PROVERD_BIND")]
    /// The interface address + port combination to accept connections on,
    /// e.g. `[::]:1234`.
    bind: String,
    #[clap(long, env = "PROVERD_LOOKUP")]
    /// A `HOSTNAME:PORT` conformant string that will be used for DNS service discovery of other nodes.
    lookup: Option<String>,
}

fn read_block(block_path: String) -> Result<Block<Transaction>, GevulotError> {
    let jblock = std::fs::read_to_string(block_path).map_err(|_| GevulotError::ErrorIo)?;
    let block: Block<Transaction> =
        serde_json::from_str(&jblock).map_err(|_| GevulotError::CanonicalDeserializeError)?;
    Ok(block)
}

#[tokio::main]
async fn main() {
    println!("Let's go!");
    let result = read_block("block.json".to_string());
    println!(
        "The block we read in looks like this:\n{:?}",
        result.unwrap()
    );
    // let config = ProverdConfig::parse();
    // env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // let shared_state = SharedState::new(SharedState::random_worker_id(), config.lookup);
    // {
    //     // start the http server
    //     let h1 = serve(&shared_state, &config.bind);

    //     // starts the duty cycle loop
    //     let ctx = shared_state.clone();
    //     // use a dedicated runtime for mixed async / heavy (blocking) compute
    //     let rt = tokio::runtime::Builder::new_multi_thread()
    //         .enable_all()
    //         .build()
    //         .unwrap();
    //     let h2 = rt.spawn(async move {
    //         loop {
    //             let ctx = ctx.clone();
    //             // enclose this call to catch panics which may
    //             // occur due to network services
    //             let _ = tokio::spawn(async move {
    //                 log::debug!("task: duty_cycle");
    //                 ctx.duty_cycle().await;
    //             })
    //             .await;
    //             tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    //         }
    //     });

    //     // this task loop makes sure to merge task results periodically
    //     // even if this instance is busy with proving
    //     let ctx = shared_state.clone();
    //     let h3 = tokio::spawn(async move {
    //         loop {
    //             let ctx = ctx.clone();
    //             // enclose this call to catch panics which may
    //             // occur due to network services
    //             let _ = tokio::spawn(async move {
    //                 log::debug!("task: merge_tasks_from_peers");
    //                 let _ = ctx.merge_tasks_from_peers().await;
    //             })
    //             .await;
    //             tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
    //         }
    //     });

    //     // wait for all tasks
    //     if tokio::try_join!(h1, h2, h3).is_err() {
    //         panic!("unexpected task error");
    //     }
    // }
}
