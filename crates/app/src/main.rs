use crate::job::{ConfigFactory, Manager};

mod api;
mod authn;
mod authz;
mod clap;
mod database;
mod frontend;
mod job;

fn main() -> anyhow::Result<()> {
    let config = job::dockerd::ConfigFactory::create();
    job::dockerd::Manager::submit(config)?;
    Ok(())
}
