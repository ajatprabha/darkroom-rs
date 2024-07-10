mod config;
mod storage;
mod error;
mod prelude;
mod handler;
mod app;

use crate::config::Config;
use crate::app::Server;
use crate::prelude::Result;

#[tokio::main]
async fn main() -> Result<()> { Server::new(&Config::new()?)?.serve().await }
