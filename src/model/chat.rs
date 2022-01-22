use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use serde::{Deserialize, Serialize};

use super::{room::Room, user::User};

#[derive(PartialEq, Eq, Debug, Clone, Serialize, Deserialize)]
pub struct SingleChat {
    user: User,
    message: String,
    timestamp: u128,
}

type TotalChat = HashMap<Room, Arc<RwLock<Vec<SingleChat>>>>;

lazy_static! {
    static ref ROOMS_LIST: Arc<RwLock<TotalChat>> = Arc::new(RwLock::new(TotalChat::new()));
}

pub fn get_chat_list(room: &Room) -> Result<Vec<SingleChat>, String> {
    let rooms_list = ROOMS_LIST.read().unwrap();
    let room_chats_unlock = rooms_list
        .get(room)
        .ok_or_else(|| "Room not found".to_string())?;
    Ok(room_chats_unlock.clone().read().unwrap().clone())
}

pub fn send_chat(room: Room, user: User, message: &str) -> Result<(), String> {
    let rooms_list = ROOMS_LIST.write().unwrap();
    let room_chats_unlock = rooms_list
        .get(&room)
        .ok_or_else(|| "Room not found".to_string())?;
    let mut room_chats_lock = room_chats_unlock.write().unwrap();
    room_chats_lock.push(SingleChat {
        user,
        message: message.to_string(),
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis(),
    });
    Ok(())
}

pub fn create_room(room: Room) -> Result<(), String> {
    let mut rooms_list = ROOMS_LIST.write().unwrap();
    if rooms_list.contains_key(&room) {
        return Err("Room existed".to_string());
    }
    rooms_list.insert(room, Arc::new(RwLock::new(vec![])));
    Ok(())
}
