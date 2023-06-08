use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};

pub async fn #name#_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    // do something before calling the next middleware

    Ok(next.run(req).await)
}
