use warp::Filter;

#[tokio::main]
async fn main() {
    let healthz_route = warp::get().map(|| warp::reply::html("{status: 'ok'}"));

    warp::serve(healthz_route).run(([0, 0, 0, 0], 8080)).await;
}
