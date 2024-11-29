#![feature(file_lock)]

mod cli;
mod cord;
mod error;
mod ipc;
mod mappings;
mod messages;
mod presence;
mod protocol;
mod session;
mod types;
mod util;

use cli::args::Args;
use cord::{Config, Cord};
use error::Result;
use util::utils::parse_client_id;

fn main() -> Result<()> {
    let args = Args::parse()?;
    let (client_id, is_custom_client) = parse_client_id(&args.client_id);

    Cord::new(Config::new(
        args.pipe_name,
        client_id,
        is_custom_client,
        args.timeout,
    ))?
    .run()
}
