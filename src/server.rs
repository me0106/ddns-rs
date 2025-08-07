use crate::{api, cli::RunArgs, service::AppCtx, website};
use axum::Router;
use tokio::{net::TcpListener, signal::ctrl_c};
use tracing::info;

pub async fn run(args: RunArgs) -> anyhow::Result<()> {
    let path = args.config();
    let ctx = AppCtx::new(path).await?;
    let listener = TcpListener::bind(&ctx.store.get_listen().await).await?;
    let router = app(ctx).await?;
    info!("server listen at {}", listener.local_addr()?);
    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown())
        .await?;
    info!("server is shutting down...");
    Ok(())
}
async fn shutdown() -> () {
    ctrl_c().await.expect("error on await ctrl_c");
}

async fn app(ctx: AppCtx) -> anyhow::Result<Router> {
    let router = Router::new()
        .merge(api::router(ctx))
        .merge(website::router());
    Ok(router)
}
