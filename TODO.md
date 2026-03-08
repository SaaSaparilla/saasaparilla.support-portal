- make job::Manager an instance impl that accepts instances of docker, kubernetes, and shell impls as well as a 
  database impl.
- make job::ManagerBuilder
- make in memory rocksdb impl
- make empty impl of job::AbstractManager for testing
- figure out how to version job submissions (do we need this?)