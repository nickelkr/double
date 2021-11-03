use warp::{Filter, Rejection, Reply};

use super::endpoint;

pub fn router(
    endpoint: endpoint::Endpoint
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let doubled = warp::path(endpoint.path.clone())
        .map(move || endpoint.payload.clone());
    let hits = warp::path("doubled")
        .and(warp::path(endpoint.path.clone()))
        .map(|| r#"{  "stats": true }"#);
    warp::get().and(
        doubled
            .or(hits)
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;

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

    #[tokio::test]
    async fn test_stats_page() {
        let endpoint = endpoint::Endpoint {
            path: "test".to_string(),
            payload: r#"{ "payload": true}"#.to_string(),
        };
        let r = router(endpoint);
        let resp = request().path("/doubled/test").reply(&r).await;
        assert_eq!(resp.status(), 200);
    }
}
