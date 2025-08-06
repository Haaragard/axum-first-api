use std::sync::Arc;

use crate::persistence::SqliteRepository;

pub mod user {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct CreateUserState {
        pub repository: Arc<SqliteRepository>,
    }
}
