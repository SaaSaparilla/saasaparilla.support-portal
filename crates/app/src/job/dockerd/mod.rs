pub struct DockerManager;
impl super::AbstractManager for DockerManager {
    fn submit(config: super::Config) -> anyhow::Result<super::Config> {
        println!("{:?}", config);
        Ok(config)
    }

    fn observe(job_id: super::ID) -> anyhow::Result<super::Config> {
        println!("{:?}", job_id);
        unimplemented!();
    }

    fn cancel(job_id: super::ID) -> anyhow::Result<super::Config> {
        println!("{:?}", job_id);
        unimplemented!();
    }
}
