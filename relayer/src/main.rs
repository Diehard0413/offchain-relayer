use clap::Parser;
use hvm_relayer::{calldata, connect, relay, runtime};
use log::info;

#[derive(Debug, Parser)]
#[command(name = "hvm-relayer", about = "offchain relayer", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, clap::Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    Connect(connect::ConnectOpts),
    #[command(arg_required_else_help = true)]
    FetchCalldata(calldata::FetchOpts),
    #[command(arg_required_else_help = true)]
    Relay(relay::RelayOpts),
}

fn main() {
    env_logger::init();
    let args = Cli::parse();

    info!("Starting hvm relayer");

    match args.command {
        Commands::Connect(opts) => {
            let rt = runtime::get_rt(opts.threads);
            rt.block_on(connect::start(opts));
        }
        Commands::FetchCalldata(opts) => {
            let rt = runtime::get_rt(opts.threads);
            rt.block_on(calldata::fetch(opts));
        }
        Commands::Relay(opts) => {
            let rt = runtime::get_rt(opts.threads);
            rt.block_on(relay::start(opts));
        }
    }
}