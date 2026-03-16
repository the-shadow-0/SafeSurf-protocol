use crate::crypto::derive_session_key;
use std::collections::HashMap;
use uuid::Uuid;
use zeroize::Zeroize;

pub struct Session {
    pub id: Uuid,
    pub session_key: [u8; 32],
}

impl Drop for Session {
    fn drop(&mut self) {
        self.session_key.zeroize();
    }
}

pub struct SessionManager {
    sessions: HashMap<Uuid, Session>,
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl SessionManager {
    pub fn new() -> Self {
        Self {
            sessions: HashMap::new(),
        }
    }

    pub fn create_session(&mut self, shared_secret: &[u8], salt: &[u8]) -> Uuid {
        let id = Uuid::new_v4();
        let session_key = derive_session_key(shared_secret, Some(salt));
        let session = Session { id, session_key };
        self.sessions.insert(id, session);
        id
    }

    pub fn get_session(&self, id: &Uuid) -> Option<&Session> {
        self.sessions.get(id)
    }

    pub fn terminate_session(&mut self, id: &Uuid) {
        self.sessions.remove(id);
    }
}
