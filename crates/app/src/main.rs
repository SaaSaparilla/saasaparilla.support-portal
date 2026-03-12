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
    let config = serde_yaml_ng::from_str(config)?;
    let mut manager = job::Manager::new(
        job::dockerd::DockerManager::new(),
        job::kubernetes::KubernetesManager::new(),
        job::shell::ShellManager::new(),
        Box::new(database::memory::MemoryDatabase::new()),
    );
    manager.submit(config)?;
    Ok(())
}
