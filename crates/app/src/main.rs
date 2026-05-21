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
        id: "placeholder"
        owner: "saasaparilla-support-portal-instance-a"
        created_at: "2026-03-08T19:45:05.709164071 UTC"
    spec:
        image: "python:3.13"
        command: "python"
        args:
            - "-c"
            - "print('hello world')"
    status:
        state: Running
        last_updated_time: "2026-03-08T19:45:05.709164071 UTC"
        message: "Container started successfully"
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
