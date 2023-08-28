use std::sync::Arc;

use sqlx::PgPool;

pub async fn start_server(web_port: u16, db: Arc<PgPool>) -> Result<(), Error> {
    let cors = warp::cors()
        .allow_origins(["http://localhost:5173"])
        .allow_headers(vec!["Auth-ID", "Content-Type", "content-type"])
        .allow_methods(vec!["GET", "POST", "HEAD", "DELETE", "PATCH"]);

    Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Web server failed to start because web-folder '{0}' not found.")]
    FailStartWebFolderNotFound(String),

    #[error{"Fail authentication missing X-Auth-Token header."}]
    FailAuthMissingXAuth,
}
