use axum::extract::FromRef;
use rustter_query::{AsyncConnection, AsyncConnectionPool, QueryError};

pub mod logging;
pub mod router;

#[derive(FromRef, Clone)]
pub struct State {
    pub db_pool: AsyncConnectionPool,
    pub signing_keys: rustter_crypto::sign::Keys,
    pub rng: rand::rngs::StdRng,
}

impl State {
    pub async fn connect(&self) -> Result<AsyncConnection, QueryError> {
        self.db_pool.get().await
    }
}

pub mod cli {
    use color_eyre::eyre::WrapErr;
    use color_eyre::Help;
    use rand_core::{CryptoRng, RngCore};
    use rustter_crypto::sign::{encode_private_key, EncodedPrivateKey, Keys};

    pub fn gen_keys<R>(rng: &mut R) -> color_eyre::Result<(EncodedPrivateKey, Keys)>
    where
        R: CryptoRng + RngCore,
    {
        let (private_key, keys) = Keys::generate(rng)?;
        let private_key = encode_private_key(private_key)?;
        Ok((private_key, keys))
    }

    pub fn load_keys() -> color_eyre::Result<Keys> {
        let private_key = std::env::var("API_PRIVATE_KEY")
            .wrap_err("failed to locate private API key")
            .suggestion("set API_PRIVATE_KEY environement variable")?;

        Ok(Keys::from_encoded(private_key)?)
    }
}
