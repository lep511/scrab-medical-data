use lambda_runtime::{run, service_fn, tracing, Error};
mod http_handler;
mod libs;
mod cds_hooks;
mod gemini;
mod llm_allergies;
mod intro_console;
mod http_page;
mod scrab_errors;
mod oidc_request;
mod oidc_database;
use http_handler::function_handler;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    run(service_fn(function_handler)).await
}
