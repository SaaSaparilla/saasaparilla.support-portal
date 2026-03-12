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
    fn save_job(&mut self, id: &job::ID, config: &job::Config) -> anyhow::Result<job::Config> {
        self.job_table.insert(id.clone(), config.clone());
        Ok(config.clone())
    }
}
