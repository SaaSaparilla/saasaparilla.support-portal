mod oidc;
mod saml;
mod credentials;

struct AuthenticationV1Request;
struct AuthenticationV1Result;

trait AuthenticationV1 {
    fn authenticate(&self, request: AuthenticationV1Request) -> Result<AuthenticationV1Result, anyhow::Error>;
}
