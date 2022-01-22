use crate::model::room::Room;
use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u16)]
pub enum ErrorCode {
    Success = 0,
    RoomExisted = 10001,
    RoomNotExisted = 10002,
}

impl Default for ErrorCode {
    fn default() -> Self {
        ErrorCode::Success
    }
}

#[derive(Default, Serialize, Deserialize)]
struct BaseResponseBody<T> {
    code: ErrorCode,
    message: String,
    data: T,
}

pub fn gene_success_room_response(message: String, room: &Room) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<&Room> {
        code: ErrorCode::Success,
        message,
        data: room,
    })
}

pub fn gene_success_rooms_response(rooms: Vec<Room>) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<Vec<Room>> {
        code: ErrorCode::Success,
        message: "success".to_string(),
        data: rooms,
    })
}

pub fn gene_error_response(error_code: ErrorCode, message: String) -> HttpResponse {
    HttpResponse::Ok().json(BaseResponseBody::<Option<String>> {
        code: error_code,
        message,
        data: None,
    })
}
