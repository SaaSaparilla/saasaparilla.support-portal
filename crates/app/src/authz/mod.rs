mod rbac;

pub struct AuthorizationV1Request;
pub struct AuthorizationV1Result;

pub trait AuthorizationV1 {
    fn authorize(
        &self,
        request: AuthorizationV1Request,
    ) -> Result<AuthorizationV1Result, anyhow::Error>;
}
