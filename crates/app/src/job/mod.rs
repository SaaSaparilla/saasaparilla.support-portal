use hifitime;
use serde::Deserialize;

pub(crate) mod dockerd;
pub(crate) mod kubernetes;
pub(crate) mod shell;

pub(crate) type ID = String; // TODO: UUID
type Argument = String;
type Command = String;
type ImageRef = String; //TODO: oci image identifier
type Message = String;
type OwnerRef = String;

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
    owner: OwnerRef,
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

pub(crate) trait DockerManager: AbstractManager {}
pub(crate) trait KubernetesManager: AbstractManager {}
pub(crate) trait ShellManager: AbstractManager {}

pub(crate) trait Database {
    fn get_job(&self, id: &ID) -> anyhow::Result<Config>;
    fn save_job(&mut self, id: &ID, config: &Config) -> anyhow::Result<Config>;
}

pub(crate) struct Manager<Do, Ku, Sh, Da> {
    dockerd: Do,
    kubernetes: Ku,
    shell: Sh,
    database: Da,
}

impl<Do: DockerManager, Ku: KubernetesManager, Sh: ShellManager, Da: Database> AbstractManager
    for Manager<Do, Ku, Sh, Da>
{
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

pub(crate) struct Unset;
pub(crate) trait DockerManagerT {}
impl DockerManagerT for Unset {}
impl<T: DockerManager> DockerManagerT for T {}
pub(crate) trait KubernetesManagerT {}
impl KubernetesManagerT for Unset {}
impl<T: KubernetesManager> KubernetesManagerT for T {}
pub(crate) trait ShellManagerT {}
impl ShellManagerT for Unset {}
impl<T: ShellManager> ShellManagerT for T {}
pub(crate) trait DatabaseT {}
impl DatabaseT for Unset {}
impl<T: Database> DatabaseT for T {}

pub(crate) struct ManagerBuilder<Do, Ku, Sh, Da>
where
    Do: DockerManagerT,
    Ku: KubernetesManagerT,
    Sh: ShellManagerT,
    Da: DatabaseT,
{
    dockerd: Do,
    kubernetes: Ku,
    shell: Sh,
    database: Da,
}
impl ManagerBuilder<Unset, Unset, Unset, Unset> {
    pub(crate) fn new() -> Self {
        Self {
            dockerd: Unset,
            kubernetes: Unset,
            shell: Unset,
            database: Unset,
        }
    }
}
impl<Do: DockerManagerT, Ku: KubernetesManagerT, Sh: ShellManagerT, Da: DatabaseT>
    ManagerBuilder<Do, Ku, Sh, Da>
{
    pub(crate) fn with_dockerd(
        self,
        dockerd: impl DockerManager + DockerManagerT,
    ) -> ManagerBuilder<impl DockerManager + DockerManagerT, Ku, Sh, Da> {
        ManagerBuilder {
            dockerd,
            kubernetes: self.kubernetes,
            shell: self.shell,
            database: self.database,
        }
    }

    pub(crate) fn with_kubernetes(
        self,
        kubernetes: impl KubernetesManager + KubernetesManagerT,
    ) -> ManagerBuilder<Do, impl KubernetesManager + KubernetesManagerT, Sh, Da> {
        ManagerBuilder {
            dockerd: self.dockerd,
            kubernetes,
            shell: self.shell,
            database: self.database,
        }
    }

    pub(crate) fn with_shell(
        self,
        shell: impl ShellManager + ShellManagerT,
    ) -> ManagerBuilder<Do, Ku, impl ShellManager + ShellManagerT, Da> {
        ManagerBuilder {
            dockerd: self.dockerd,
            kubernetes: self.kubernetes,
            shell,
            database: self.database,
        }
    }

    pub(crate) fn with_database(
        self,
        database: impl Database + DatabaseT,
    ) -> ManagerBuilder<Do, Ku, Sh, impl Database + DatabaseT> {
        ManagerBuilder {
            dockerd: self.dockerd,
            kubernetes: self.kubernetes,
            shell: self.shell,
            database,
        }
    }
}
impl<
    Do: DockerManager + DockerManagerT,
    Ku: KubernetesManager + KubernetesManagerT,
    Sh: ShellManager + ShellManagerT,
    Da: Database + DatabaseT,
> ManagerBuilder<Do, Ku, Sh, Da>
{
    pub(crate) fn build(self) -> impl AbstractManager {
        Manager {
            dockerd: self.dockerd,
            kubernetes: self.kubernetes,
            shell: self.shell,
            database: self.database,
        }
    }
}
