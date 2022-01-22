use actix_web::web;
mod room;
mod user;
mod utils;
use room::{create_room, delete_room, get_room};
use user::{login, logout};

pub fn router_config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_room)
        .service(delete_room)
        .service(get_room)
        .service(login)
        .service(logout);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::cookie::Cookie;
    use actix_http::Request;
    use actix_session::CookieSession;
    use actix_web::dev::{Service, ServiceResponse};
    use actix_web::http::Method;
    use actix_web::{http, test, App, Error};

    // ref cannot ref to local variable, so take out this function which return a value
    async fn get_resp<'a>(
        app: &'a mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
        method: Method,
        uri: &str,
        coookies: Option<Vec<Cookie<'a>>>,
    ) -> ServiceResponse {
        let mut req_pre = test::TestRequest::with_uri(uri).method(method);
        if let Some(cookies) = coookies {
            for cookie in cookies {
                req_pre = req_pre.cookie(cookie);
            }
        }
        let req = req_pre.to_request();
        app.call(req).await.unwrap()
    }

    fn get_cookies_from_resp(resp: &ServiceResponse) -> Vec<Cookie> {
        let mut cookies = Vec::new();
        for cookie in resp.response().cookies() {
            cookies.push(cookie.clone());
        }
        cookies
    }

    async fn assert_resp(
        app: &mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
        method: Method,
        uri: &str,
        expected: &str,
    ) -> ServiceResponse {
        let resp = get_resp(app, method, uri, None).await;

        //  = _resp.response();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        assert_eq!(response_body, expected);
        resp
    }

    async fn assert_resp_with_cookies<'a>(
        app: &'a mut impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
        method: Method,
        uri: &str,
        cookies: Vec<Cookie<'a>>,
        expected: &str,
    ) -> ServiceResponse {
        let resp = get_resp(app, method, uri, Some(cookies)).await;
        //  = _resp.response();
        assert_eq!(resp.status(), http::StatusCode::OK);
        let response_body = match resp.response().body().as_ref() {
            Some(actix_web::body::Body::Bytes(bytes)) => bytes,
            _ => panic!("Response error"),
        };
        assert_eq!(response_body, expected);
        resp
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

    #[actix_rt::test]
    async fn test_login() -> Result<(), Error> {
        let app = App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .configure(router_config);
        let mut app = test::init_service(app).await;
        let resp = assert_resp(
            &mut app,
            Method::POST,
            "/user/login/eki",
            r##"{"code":0,"message":"User login successfully","data":{"id":"eki","ip":"unkown_ip","token":2246886105278960331}}"##,
        )
        .await;
        let cookies = get_cookies_from_resp(&resp);
        assert_resp(
            &mut app,
            Method::POST,
            "/user/logout",
            r##"{"code":10004,"message":"No user field in cookies","data":null}"##,
        )
        .await;
        assert_resp_with_cookies(
            &mut app,
            Method::POST,
            "/user/logout",
            cookies,
            r##"{"code":0,"message":"User logout successfully","data":{"id":"eki","ip":"unkown_ip","token":2246886105278960331}}"##,
        )
        .await;
        Ok(())
    }
}
