use warp::test::request;
use warp::{Filter, Rejection, Reply};

use super::endpoint;

pub fn router(
    endpoint: endpoint::Endpoint
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path(endpoint.path)
        .map(move || endpoint.payload.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_router() {
        let endpoint = endpoint::Endpoint {
            path: "test".to_string(),
            payload: r#"{ "payload": true }"#.to_string(),
        };
        let r = router(endpoint);
        let resp = request().path("/test").reply(&r).await;
        assert_eq!(resp.status(), 200);
        assert_eq!(resp.body(), r#"{ "payload": true }"#);
    }
}
