use super::utils::{gene_error_response, gene_success_user_response, ErrorCode};
use crate::model::user;
use crate::utils::format_and_debug_msg;
use actix_session::Session;
use actix_web::{post, web, HttpRequest, HttpResponse};
use std::error::Error;

// use anyhow::Result;

#[post("user/login/{user_id}")]
async fn login(
    session: Session,
    req: HttpRequest,
    web::Path(user_id): web::Path<String>,
) -> HttpResponse {
    let connection_info = req.connection_info();
    let user_ip = connection_info.realip_remote_addr().unwrap_or("unkown_ip");
    let new_user = user::create_user(&user_id, user_ip);

    session
        .set("user", &new_user)
        .map(|_| {
            let message = format_and_debug_msg!("User login successfully");
            gene_success_user_response(message, &new_user)
        })
        .unwrap_or_else(|message| gene_error_response(ErrorCode::LoginError, message.to_string()))
}

#[post("user/logout")]
async fn logout(session: Session) -> HttpResponse {
    let logout_result = _logout(session);
    logout_result.unwrap_or_else(|e| gene_error_response(ErrorCode::LogoutError, e.to_string()))
}

fn _logout(session: Session) -> Result<HttpResponse, Box<dyn Error>> {
    let old_user = session
        .get::<user::User>("user")?
        .ok_or("No user field in cookies")?;
    let token = user::calc_user_token(&old_user.id(), &old_user.ip());
    if token != *old_user.token() {
        Err("User token is not match")?;
    }
    session.remove("user");
    let message = format_and_debug_msg!("User logout successfully");
    Ok(gene_success_user_response(message, &old_user))
}
