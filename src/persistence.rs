use crate::{entity::user::User, error::{Error, Result}};

#[derive(Clone, Debug)]
pub struct SqliteRepository {}
impl SqliteRepository {
    pub fn new(_path: String) -> Self {
        Self {}
    }

    pub fn create_user(&self, input: CreateUserInputDTO) -> Result<CreateUserOutputDTO, Error> {
        let user = User::new(
            Some("id-123-456-789".into()),
            input.name,
        );
        if user.id.is_none() {
            return Err(Error::UserCreateCouldNotCreateUser);
        }

        Ok(CreateUserOutputDTO {
            id: user.id.unwrap(),
            name: user.name,
        })
    }
}

pub struct CreateUserInputDTO {
    pub name: String,
}

pub struct CreateUserOutputDTO {
    pub id: String,
    pub name: String,
}
