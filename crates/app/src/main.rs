use serde_yaml_ng;

mod api;
mod authn;
mod authz;
mod clap;
mod database;
mod frontend;
mod job;

use job::AbstractManager;

fn main() -> anyhow::Result<()> {
    let config = r#"
    kind: DockerD
    metadata:
        name: "docker-job"
    spec:
        image: "python:3.13"
        command: "python"
        args:
            - "-c"
            - "print('hello world')"
    "#;
    let config: job::Config = serde_yaml_ng::from_str(config)?;
    let mut job_manager = job::ManagerBuilder::new()
        .with_dockerd(job::dockerd::DockerManager::new())
        .with_kubernetes(job::kubernetes::KubernetesManager::new())
        .with_shell(job::shell::ShellManager::new())
        .with_database(database::memory::MemoryDatabase::new())
        .build();
    job_manager.submit(config)?;
    Ok(())
}
