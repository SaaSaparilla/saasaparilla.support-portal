use crate::job::{Config, ID};

pub struct Spec {
    jobSpec: String,
}
pub struct Status {}

pub struct ConfigFactory;
impl super::ConfigFactory for ConfigFactory {
    fn create() -> super::Config {
        super::Config {
            metadata: super::Metadata {
                id: "placeholder".into(),
                kind: super::Platform::Kubernetes,
            },
            spec: super::Spec::Kubernetes(Spec {
                jobSpec: "placeholder".into(),
            }),
            status: None,
        }
    }
}

pub struct Manager;
impl super::Manager for Manager {
    fn submit(config: Config) -> crate::job::Result<Config> {
        unimplemented!();
    }

    fn observe(job_id: ID) -> crate::job::Result<Config> {
        unimplemented!();
    }

    fn cancel(job_id: ID) -> crate::job::Result<Config> {
        unimplemented!();
    }
}
