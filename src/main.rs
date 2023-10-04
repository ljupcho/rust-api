use anyhow::Result;
use rust_api::AppRouter;

#[tokio::main]
async fn main() -> Result<()> {
    let db = rust_api::shared::db::DB::connect().await?;
    // let store_posts = PostStore::new(&db);
    // let srv_posts = PostService::new(&store_posts.clone());
    // let store_posts = Arc::new(Mutex::new(PostStore::new(&db)));
    // let srv_posts = PostService::new(store_posts);
    
    // let router_posts = Router::new()
    //         .route("/posts", get(rust_api::handlers::posts::get_posts))
    //         .route("/posts/:id", get(rust_api::handlers::posts::get_post))
    //         .with_state(srv_posts);
    //
    // let router = Router::new()
    //         .nest("/api", router_posts);
    
    // Build app state
    // let state = AppState {
    //     database: db,
    // };

    let router = AppRouter::build(db);

    // let router = Router::new()
    //     .route("/api/postsnew/:id", get(handlers::posts::get_post_new)) 
    //     .with_state(state)
    // ;

    // Run our application as a hyper server on http://localhost:9900.
    axum::Server::bind(&"0.0.0.0:9900".parse().unwrap())
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

/// Tokio signal handler that will wait for a user to press CTRL+C.
/// We use this in our hyper `Server` method `with_graceful_shutdown`.
async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("signal shutdown");
}
