struct SubmitJobV1Request;
struct SubmitJobV1Response;

trait SubmitJobV1 {
    fn submit_job(&self, job: SubmitJobV1Request) -> Result<SubmitJobV1Response, anyhow::Error>;
}

struct GetJobStatusV1Request;
struct GetJobStatusV1Response;

trait GetJobStatusV1 {
    fn get_job_status(
        &self,
        job: GetJobStatusV1Request,
    ) -> Result<GetJobStatusV1Response, anyhow::Error>;
}

struct CancelJobV1Request;
struct CancelJobV1Response;

trait CancelJobV1 {
    fn cancel_job(&self, job: CancelJobV1Request) -> Result<CancelJobV1Response, anyhow::Error>;
}
