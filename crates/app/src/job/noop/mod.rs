pub struct NoopManager;
impl NoopManager {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
pub(super) const NOOP_CONFIG: super::Config = super::Config {
    kind: super::Platform::DockerD,
    metadata: super::Metadata {
        id: Some(String::new()),
        name: String::new(),
        owner: None,
        created_at: None,
    },
    spec: super::Spec {
        image: String::new(),
        command: String::new(),
        args: vec![],
    },
    status: None,
};
impl super::AbstractManager for NoopManager {
    fn submit(&mut self, config: super::Config) -> anyhow::Result<super::Config> {
        println!("{:?}", config);
        Ok(config)
    }

    fn observe(&mut self, job_id: super::ID) -> anyhow::Result<super::Config> {
        Ok(NOOP_CONFIG)
    }

    fn cancel(&mut self, job_id: super::ID) -> anyhow::Result<super::Config> {
        Ok(NOOP_CONFIG)
    }
}
impl super::DockerManager for NoopManager {}
impl super::KubernetesManager for NoopManager {}
impl super::ShellManager for NoopManager {}
