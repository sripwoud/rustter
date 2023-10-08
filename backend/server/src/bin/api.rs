use clap::{command, Parser, Subcommand};
use color_eyre::eyre::Context;
use color_eyre::{Help, Result};
use std::net::SocketAddr;
use tracing::{debug, error, info};

#[derive(Debug, Subcommand)]
enum Command {
    /// Generate a private session signing key
    GenKey,
}
#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[clap(
        short,
        long,
        default_value = "postgres://test@localhost/test",
        env = "API_DATABASE_URL"
    )]
    database_url: String,

    #[clap(short, long, default_value = "127.0.0.1:8070", env = "API_BIND")]
    bind: SocketAddr,

    #[clap(flatten)]
    verbosity: rustter_server::logging::Verbosity,

    #[clap(subcommand)]
    command: Option<Command>,
}

async fn run() -> Result<()> {
    color_eyre::install()?;

    let use_dotenv = dotenvy::dotenv();
    let args = Cli::parse();
    rustter_server::logging::setup(args.verbosity);
    if let Ok(path) = use_dotenv {
        debug!(target: "rustter_server", dot_env_found = true, path = %path.to_string_lossy());
    } else {
        debug!(target: "rustter_server", dot_env_found = false);
    }

    if let Some(command) = args.command {
        match command {
            Command::GenKey => {
                let mut rng = rustter_crypto::new_rng();

                info!(target: "rustter_server", "generating private key...");
                let (key, _) = rustter_server::cli::gen_keys(&mut rng)?;

                let path = "private_key.base64";
                std::fs::write(path, key.as_str())?;
                info!(target: "rustter_server", path=path, "private key written to disk");

                info!(target: "rustter_server", "set API_PRIVATE_KEY environment variable to the contents of {} in order to use it", path);

                return Ok(());
            }
        }
    }

    debug!(target: "rustter_server", "loading private key...");
    let signing_keys = rustter_server::cli::load_keys()?;

    info!(target: "rustter_server", database_url=args.database_url, "connecting to database...");
    let db_pool = rustter_query::AsyncConnectionPool::new(&args.database_url)
        .await
        .with_suggestion(|| "check database URL")
        .with_suggestion(|| "ensure correct database access rights")
        .with_suggestion(|| "ensure database exists")?;

    let state = rustter_server::AppState {
        db_pool,
        signing_keys,
        rng: rustter_crypto::new_rng(),
    };

    info!(target: "rustter_server", bind = %args.bind);

    let router = rustter_server::router::new_router(state);
    let server = axum::Server::try_bind(&args.bind)
        .wrap_err_with(|| "server initialization failed")
        .with_suggestion(|| "check bind address")
        .with_suggestion(|| "check if port is already in use")?;
    let server = server.serve(router.into_make_service());
    info!(target: "rustter_server", "server listening");

    if let Err(e) = server.await {
        error!(target: "rustter_server", server_error=%e);
    }

    Ok(())
}
#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
