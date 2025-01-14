/// A `Credential` stores a users email and password linked to their supercell
/// API account. `Credentials` is a simple wrapper around a list of `Credential`
/// objects. It comes with helper functions `add_credential` and `build` to
/// add Credential objects into the list of credentials and build the object.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Credential {
    pub email: String,
    pub password: String,
}

impl Credential {
    #[must_use]
    pub fn email(&self) -> &str {
        &self.email
    }

    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Clone)]
pub struct Credentials(pub Vec<Credential>);

impl Credentials {
    #[must_use]
    pub const fn builder() -> CredentialsBuilder {
        CredentialsBuilder::new()
    }

    #[must_use]
    pub const fn empty() -> Self {
        Self(Vec::new())
    }
}

#[derive(Debug)]
pub struct CredentialsBuilder {
    credentials: Credentials,
}

impl CredentialsBuilder {
    const fn new() -> Self {
        Self {
            credentials: Credentials(Vec::new()),
        }
    }

    #[must_use]
    pub fn add_credential(mut self, email: String, password: String) -> Self {
        self.credentials.0.push(Credential { email, password });
        self
    }

    #[must_use]
    pub fn build(self) -> Credentials {
        self.credentials
    }
}
