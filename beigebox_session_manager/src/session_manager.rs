use std::sync::{Arc};
use std::thread;
use std::time::{Duration, SystemTime};

use uuid::Uuid;

use dashmap::DashMap;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Session {
    uuid: Uuid,
    created: SystemTime
} impl Session {
    pub fn generate() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            created: SystemTime::now()
        }
    }
    
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }
    
    pub fn created(&self) -> &SystemTime {
        &self.created
    }
}

pub struct SessionManager {
    sessions: Arc<DashMap<Uuid, (Session, String)>>,
} impl SessionManager {
    pub fn new() -> Self {
        let sessions: Arc<DashMap<Uuid, (Session, String)>> = Arc::new(DashMap::new());
        
        // Automatically remove expired sessions every 60 seconds.
        let cleanup_sessions = sessions.clone();
        thread::spawn(move || { 
            loop {
                thread::sleep(Duration::from_secs(60));

                let expired_keys: Vec<_> = cleanup_sessions.iter()
                    .filter(|entry| entry.value().0.created.elapsed().map_or(true, |e| e > Duration::from_secs(30*60)))
                    .map(|entry| entry.key().clone())
                    .collect();

                for key in expired_keys {
                    cleanup_sessions.remove(&key);
                }
            }
        });
        
        Self {
            sessions
        }
    }
    
    /// Generate a session for a username.
    /// 
    /// This session will be added to the sessions hashmap along with the username.
    pub fn generate_session(&self, username: &str) -> Session {
        let mut session = Session::generate();
        
        // This loop ensures the created UUID doesn't match any other UUID. Theoretically it should 
        // take billions of UUID creations to ever reach this point but who knows.
        while self.sessions.contains_key(&session.uuid) {
            session = Session::generate();
        }
        
        self.sessions.insert(session.uuid, (session.clone(), username.to_owned()));
        
        session
    }
    
    /// Find session with O(1) complexity.
    pub fn get_session_by_uuid(&self, uuid: Uuid) -> Option<(Session, String)> {
        self.sessions.get(&uuid).map(|entry| entry.clone())
    }
}
