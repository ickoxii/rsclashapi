// Declare submodules
pub mod auth;
pub mod models;
pub mod utils;

// Declare modules in this current directory
pub mod api;
pub mod error;

#[cfg(test)]
mod test {
    use dotenv::dotenv;
    use std::env;

    use crate::auth::dev::{APIAccount, LoginResponse};
    use crate::auth::credentials::Credentials;

    use crate::api::api;

    use anyhow;

    use reqwest;
    use serde_json::{from_str, Value};

    fn get_credentials() -> Credentials {
        dotenv().ok();

        let email = env::var("EMAIL").expect("EMAIL environment variable missing");
        let password = env::var("PASSWORD").expect("PASSWORD environment variable missing");

        Credentials::builder()
            .add_credential(email, password)
            .build()
    }

    /*
    #[tokio::test]
    async fn test_login() -> anyhow::Result<()> {
        let credentials: Credentials = get_credentials();

        println!("{:#?}", credentials);

        let email = credentials
            .0
            .first()
            .ok_or_else(|| anyhow::anyhow!("No credentials found"))?
            .email();

        let password = credentials
            .0
            .first()
            .ok_or_else(|| anyhow::anyhow!("No credentials found"))?
            .password();

        let login_response = APIAccount::login(email, password).await?;

        println!("{:#?}", login_response);

        Ok(())
    }
    */
}
