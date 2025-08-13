use std::sync::Arc;

use crate::persistence::UserRepository;

pub mod user {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct CreateUserState {
        pub user_repository: Arc<dyn UserRepository>,
    }
}
