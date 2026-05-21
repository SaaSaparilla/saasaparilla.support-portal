use crate::job;
use std::collections::HashMap;

pub(crate) struct MemoryDatabase {
    job_table: HashMap<job::ID, job::Config>,
}
impl MemoryDatabase {
    pub(crate) fn new() -> Self {
        Self {
            job_table: Default::default(),
        }
    }
}
impl job::Database for MemoryDatabase {
    fn get_job(&self, id: &job::ID) -> anyhow::Result<job::Config> {
        self.job_table
            .get(id)
            .cloned()
            .ok_or(anyhow::anyhow!("Job not found"))
    }
    // TODO: less clones
    // TODO: also, definitely split serialization for manager and database
    // TODO: probably do it with a dao
    fn save_job(&mut self, config: &job::Config) -> anyhow::Result<job::Config> {
        self.job_table.insert(
            config
                .metadata
                .id
                .clone()
                .ok_or(anyhow::anyhow!(
                    "id should not be empty when submitting to database"
                ))?
                .clone(),
            config.clone(),
        );
        Ok(config.clone())
    }
}
