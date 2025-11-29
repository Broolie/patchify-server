use std::{collections::HashMap, sync::Arc};

use anyhow::Result;

use crate::errors::Errors;

pub struct Session {}

impl Session {
    fn new() -> Self {
        Session {}
    }
}

pub struct SessionManager {
    store: HashMap<usize, Arc<Session>>,
}

impl SessionManager {
    pub fn new() -> Self {
        SessionManager {
            store: HashMap::new(), //TODO: maybe we should create store with some capacity, like 100?
        }
    }

    pub fn create_session(&mut self, id: usize) -> Result<Arc<Session>> {
        if self.store.contains_key(&id) {
            return Err(Errors::SessionAlreadyExist.into())
        }

        Ok(self.store.entry(id).or_insert(Arc::new(Session::new())).clone())
    }
}
