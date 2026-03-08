use hifitime;
use serde::Deserialize;

pub mod dockerd;
pub mod kubernetes;

type ID = String; // TODO: UUID
type Message = String;
type ImageRef = String; //TODO: oci image identifier
type Command = String;
type Argument = String;

#[derive(Debug, Deserialize)]
pub struct Config {
    kind: Platform,
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>,
}

#[derive(Debug, Deserialize)]
enum Platform {
    DockerD,
    Kubernetes,
    Shell,
}

#[derive(Debug, Deserialize)]
pub struct Metadata {
    id: ID,
    created_at: hifitime::Epoch,
}

#[derive(Debug, Deserialize)]
pub struct Spec {
    image: ImageRef,
    command: Command,
    args: Vec<Argument>,
}

#[derive(Debug, Deserialize)]
pub struct Status {
    state: RuntimeState,
    last_updated_time: hifitime::Epoch,
    message: Message,
}

#[derive(Debug, Deserialize)]
enum RuntimeState {
    Queued,
    Running,
    Cancelled,
    Succeeded,
    Failed,
}

pub trait AbstractManager {
    fn submit(config: Config) -> anyhow::Result<Config>;
    fn observe(job_id: ID) -> anyhow::Result<Config>;
    fn cancel(job_id: ID) -> anyhow::Result<Config>;
}

pub struct Manager;
impl Manager {
    pub fn submit(config: Config) -> anyhow::Result<Config> {
        <Self as AbstractManager>::submit(config)
    }
    pub fn observe(job_id: ID) -> anyhow::Result<Config> {
        <Self as AbstractManager>::observe(job_id)
    }
    pub fn cancel(job_id: ID) -> anyhow::Result<Config> {
        <Self as AbstractManager>::cancel(job_id)
    }
}

impl AbstractManager for Manager {
    fn submit(config: Config) -> anyhow::Result<Config> {
        match config.kind {
            Platform::DockerD => dockerd::DockerManager::submit(config),
            Platform::Kubernetes => kubernetes::KubernetesManager::submit(config),
            Platform::Shell => todo!(),
        }
    }

    fn observe(job_id: ID) -> anyhow::Result<Config> {
        todo!("retrieve job definition and match to the correct impl");
        todo!("return a stream of updates")
    }

    fn cancel(job_id: ID) -> anyhow::Result<Config> {
        todo!("retrieve job definition and match to the correct impl")
    }
}
