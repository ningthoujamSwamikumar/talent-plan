use std::collections::HashMap;

use crate::connection::Connection;
use crate::crdt::LwwRegister;
use crate::message::ServerMessage;

/// A chat/collaboration room that holds its members and shared state.
#[derive(Debug)]
pub struct Room {
    pub id: String,
    pub members: HashMap<String, Connection>,
    /// LWW registers keyed by (task_id, field).
    pub registers: HashMap<(String, String), LwwRegister<String>>,
}

impl Room {
    /// Create a new empty room.
    pub fn new(id: String) -> Self {
        todo!()
    }

    /// Add a connection to this room. Returns the current member list.
    pub fn add_member(&mut self, _conn: Connection) -> Vec<String> {
        todo!()
    }

    /// Remove a member by user_id. Returns true if the member was present.
    pub fn remove_member(&mut self, _user_id: &str) -> bool {
        todo!()
    }

    /// Return a list of user_ids currently in the room.
    pub fn member_ids(&self) -> Vec<String> {
        todo!()
    }

    /// Broadcast a server message to all members of the room.
    pub fn broadcast(&self, _msg: ServerMessage) {
        todo!()
    }

    /// Broadcast a server message to all members except `exclude_user_id`.
    pub fn broadcast_except(&self, _msg: ServerMessage, _exclude_user_id: &str) {
        todo!()
    }

    /// Apply an edit via the LWW register and return the current value.
    pub fn apply_edit(
        &mut self,
        _task_id: String,
        _field: String,
        _value: String,
        _timestamp: i64,
        _writer_id: String,
    ) -> (String, i64) {
        todo!()
    }
}
