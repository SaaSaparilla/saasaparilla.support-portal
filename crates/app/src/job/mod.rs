pub mod dockerd;
pub mod kubernetes;

// This approach isn't going to work.  We're going to have to figure out what our options for
// arbitrary depth n-ary trees of unknown leaf type are unless we want to parse our strings
// directly in this module.  Maybe that is what we'll have to do in order to support arbitrary
// job definitions that don't require recompilation like backstage

type Result<T> = anyhow::Result<T>;
type Error = anyhow::Error;
type ID = String; // TODO: UUID

enum Platform {
    DockerD,
    Kubernetes,
}

enum Spec {
    DockerD(dockerd::Spec),
    Kubernetes(kubernetes::Spec),
}

enum Status {
    DockerD(dockerd::Status),
    Kubernetes(kubernetes::Status),
}

pub struct Metadata {
    id: ID,
    kind: Platform,
}

pub struct Config {
    metadata: Metadata,
    spec: Spec,
    status: Option<Status>,
}

//TODO: make individual executor impls private and only take in a json string or similar struct
pub trait ConfigFactory {
    fn create() -> Config;
    //TODO: from<types>
}

pub trait Manager {
    fn submit(config: Config) -> Result<Config>;
    fn observe(job_id: ID) -> Result<Config>;
    fn cancel(job_id: ID) -> Result<Config>;
}
