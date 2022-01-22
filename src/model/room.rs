use crate::utils::format_and_debug_msg;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

lazy_static! {
    static ref ROOMS_LIST: Arc<RwLock<Vec<Room>>> = Arc::new(RwLock::new(vec![]));
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Room {
    id: String,
}

pub fn get_rooms() -> Vec<Room> {
    ROOMS_LIST.read().unwrap().clone()
}

pub fn create_room(room_id: &str) -> Result<Room, String> {
    let mut rooms_list = ROOMS_LIST.write().unwrap();
    if rooms_list.iter().any(|x| x.id == room_id) {
        let message = format_and_debug_msg!("Room {} existed", room_id);
        return Err(message);
    }
    let new_room = Room {
        id: room_id.to_string(),
    };
    rooms_list.push(new_room.clone());
    Ok(new_room)
}

pub fn delete_room(room_id: &str) -> Result<Room, String> {
    let mut rooms_list = ROOMS_LIST.write().unwrap();
    if !rooms_list.iter().any(|x| x.id == room_id) {
        let message = format_and_debug_msg!("Room {} not existed", room_id);
        return Err(message);
    }
    let room_index = rooms_list.iter().position(|x| x.id == room_id).unwrap();
    Ok(rooms_list.remove(room_index))
}
