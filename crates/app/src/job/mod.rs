use hifitime;
use serde::Deserialize;

pub(crate) mod dockerd;
pub(crate) mod kubernetes;
pub(crate) mod shell;

pub(crate) type ID = String; // TODO: UUID
type Message = String;
type ImageRef = String; //TODO: oci image identifier
type Command = String;
type Argument = String;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    kind: Platform,
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>,
}

#[derive(Clone, Debug, Deserialize)]
enum Platform {
    DockerD,
    Kubernetes,
    Shell,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Metadata {
    id: ID,
    created_at: hifitime::Epoch,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Spec {
    image: ImageRef,
    command: Command,
    args: Vec<Argument>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Status {
    state: RuntimeState,
    last_updated_time: hifitime::Epoch,
    message: Message,
}

#[derive(Clone, Debug, Deserialize)]
enum RuntimeState {
    Queued,
    Running,
    Cancelled,
    Succeeded,
    Failed,
}

pub(crate) trait AbstractManager {
    fn submit(&mut self, config: Config) -> anyhow::Result<Config>;
    //TODO: separate this into `logs()`, `status()` and other actually useful abstractions
    fn observe(&mut self, job_id: ID) -> anyhow::Result<Config>;
    fn cancel(&mut self, job_id: ID) -> anyhow::Result<Config>;
}

pub(crate) trait Database {
    fn get_job(&self, id: &ID) -> anyhow::Result<Config>;
    fn save_job(&mut self, id: &ID, config: &Config) -> anyhow::Result<Config>;
}

pub(crate) struct Manager {
    dockerd: dockerd::DockerManager,
    kubernetes: kubernetes::KubernetesManager,
    shell: shell::ShellManager,
    database: Box<dyn Database>,
}

impl Manager {
    //TODO: make this private and only access it through a builder
    pub(crate) fn new(
        dockerd: dockerd::DockerManager,
        kubernetes: kubernetes::KubernetesManager,
        shell: shell::ShellManager,
        database: Box<dyn Database>,
    ) -> Self {
        Self {
            dockerd,
            kubernetes,
            shell,
            database,
        }
    }
}

impl AbstractManager for Manager {
    fn submit(&mut self, config: Config) -> anyhow::Result<Config> {
        self.database.save_job(&config.metadata.id, &config)?;
        let result = match config.kind {
            Platform::DockerD => self.dockerd.submit(config)?,
            Platform::Kubernetes => self.kubernetes.submit(config)?,
            Platform::Shell => self.shell.submit(config)?,
        };
        self.database.save_job(&result.metadata.id, &result)?;
        Ok(result)
    }

    fn observe(&mut self, job_id: ID) -> anyhow::Result<Config> {
        let config = self.database.get_job(&job_id)?;
        let result = match config.kind {
            Platform::DockerD => self.dockerd.observe(job_id)?,
            Platform::Kubernetes => self.kubernetes.observe(job_id)?,
            Platform::Shell => self.shell.observe(job_id)?,
        };
        self.database.save_job(&result.metadata.id, &result)?;
        Ok(result)
    }

    fn cancel(&mut self, job_id: ID) -> anyhow::Result<Config> {
        let config = self.database.get_job(&job_id)?;
        let result = match config.kind {
            Platform::DockerD => self.dockerd.cancel(job_id)?,
            Platform::Kubernetes => self.kubernetes.cancel(job_id)?,
            Platform::Shell => self.shell.cancel(job_id)?,
        };
        self.database.save_job(&result.metadata.id, &result)?;
        Ok(result)
    }
}
