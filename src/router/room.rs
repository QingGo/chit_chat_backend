use super::utils::{
    gene_error_response, gene_success_room_response, gene_success_rooms_response, ErrorCode,
};
use crate::model::room;
use crate::utils::format_and_debug_msg;
use actix_web::{delete, get, post, web, HttpResponse, Responder};

#[post("room/{room_id}")]
async fn create_room(web::Path(room_id): web::Path<String>) -> HttpResponse {
    let new_room = room::create_room(&room_id);
    new_room
        .map(|room| {
            let message = format_and_debug_msg!("Room {} created", room_id);
            gene_success_room_response(message, &room)
        })
        .unwrap_or_else(|message| gene_error_response(ErrorCode::RoomExisted, message))
}

#[delete("room/{room_id}")]
async fn delete_room(web::Path(room_id): web::Path<String>) -> HttpResponse {
    let deleted_room = room::delete_room(&room_id);
    deleted_room
        .map(|room| {
            let message = format_and_debug_msg!("Delete room: {}", room_id);
            gene_success_room_response(message, &room)
        })
        .unwrap_or_else(|message| gene_error_response(ErrorCode::RoomNotExisted, message))
}

#[get("room/list")]
async fn get_room() -> impl Responder {
    let rooms = room::get_rooms();
    debug!("list rooms: {:?}", rooms);
    gene_success_rooms_response(rooms)
}
