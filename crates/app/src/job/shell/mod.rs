pub struct ShellManager;
impl ShellManager {
    pub(crate) fn new() -> Self {
        Self {}
    }
}
impl super::AbstractManager for ShellManager {
    fn submit(&mut self, config: super::Config) -> anyhow::Result<super::Config> {
        println!("{:?}", config);
        Ok(config)
    }

    fn observe(&mut self, job_id: super::ID) -> anyhow::Result<super::Config> {
        println!("{:?}", job_id);
        unimplemented!();
    }

    fn cancel(&mut self, job_id: super::ID) -> anyhow::Result<super::Config> {
        println!("{:?}", job_id);
        unimplemented!();
    }
}
impl super::ShellManager for ShellManager {}
