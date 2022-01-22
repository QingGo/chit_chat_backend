use actix_web::web;
mod room;
mod utils;
use room::{create_room, delete_room, get_room};

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_room)
        .service(delete_room)
        .service(get_room);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::Request;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::http::Method;
    use actix_web::{http, test, App, Error};

    async fn assert_resp(
        app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
        method: Method,
        uri: &str,
        expected: &str,
    ) {
        let req = test::TestRequest::with_uri(uri).method(method).to_request();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        assert_eq!(response_body, expected);
    }

    #[actix_rt::test]
    async fn test_rooms_modify() -> Result<(), Error> {
        let app = App::new().configure(router_config);
        let mut app = test::init_service(app).await;
        assert_resp(
            &mut app,
            Method::POST,
            "/room/room1",
            r##"{"code":0,"message":"Room room1 created","data":{"id":"room1"}}"##,
        )
        .await;
        assert_resp(
            &mut app,
            Method::POST,
            "/room/room1",
            r##"{"code":10001,"message":"Room room1 existed","data":null}"##,
        )
        .await;
        assert_resp(
            &mut app,
            Method::POST,
            "/room/room2",
            r##"{"code":0,"message":"Room room2 created","data":{"id":"room2"}}"##,
        )
        .await;
        assert_resp(
            &mut app,
            Method::GET,
            "/room/list",
            r##"{"code":0,"message":"success","data":[{"id":"room1"},{"id":"room2"}]}"##,
        )
        .await;
        assert_resp(
            &mut app,
            Method::DELETE,
            "/room/room1",
            r##"{"code":0,"message":"Delete room: room1","data":{"id":"room1"}}"##,
        )
        .await;
        assert_resp(
            &mut app,
            Method::DELETE,
            "/room/room1",
            r##"{"code":10002,"message":"Room room1 not existed","data":null}"##,
        )
        .await;
        assert_resp(
            &mut app,
            Method::GET,
            "/room/list",
            r##"{"code":0,"message":"success","data":[{"id":"room2"}]}"##,
        )
        .await;
        Ok(())
    }
}
