use std::{fmt::Debug, sync::{Arc, Mutex}};

use chrono::Utc;
use rusqlite::Connection;

use crate::{entity::user::User, error::{Error, Result}};

pub trait UserRepository: Debug + Send + Sync {
    fn create_table(&self) -> Result<(), Error>;
    fn create_user(&self, input: CreateUserInputDTO) -> Result<CreateUserOutputDTO, Error>;
}

#[derive(Clone, Debug)]
pub struct SqliteRepository {
    conn: Arc<Mutex<Connection>>,
}
impl SqliteRepository {
    pub fn new(path: String) -> Result<Self, Error> {
        let conn = Arc::new(Mutex::new({
            match path.eq(":memory") {
                true => Connection::open_in_memory().expect("Could not open SQLite database."),
                false => Connection::open(path)?
            }
        }));

        let instance = Self { conn };
        instance.create_table()?;

        Ok(instance)
    }
}
impl UserRepository for SqliteRepository {
    fn create_table(&self) -> Result<(), Error> {
        if let Ok(conn) = self.conn.lock() {
            conn.execute(
                "CREATE TABLE IF NOT EXISTS users (
                    id          TEXT PRIMARY KEY,
                    name        TEXT NOT NULL,
                    created_at  DATETIME NOT NULL
                )",
                []
            )?;

            return Ok(());
        }

        Err(Error::ConnectionLockError)
    }

    fn create_user(&self, input: CreateUserInputDTO) -> Result<CreateUserOutputDTO, Error> {
        let user_guid = guid_create::GUID::rand().to_string();
        let created_at = Utc::now().to_rfc3339();

        let result = self.conn.lock().unwrap().execute(
            "INSERT INTO users (id, name, created_at) VALUES (?1, ?2, ?3)",
            (&user_guid, &input.name, &created_at)
        );

        match result {
            Ok(_) => Ok(CreateUserOutputDTO {
                user: User::new(
                    Some(user_guid),
                    input.name,
                )
            }),
            _ => Err(Error::UserCreateCouldNotCreateUser)
        }
    }
}

#[derive(Clone, Debug)]
pub struct MysqlRepository {
    conn: String,
}
impl MysqlRepository {
    pub fn new(
        _path: String,
        _user: String,
        _password: String
    ) -> Result<Self, Error> {
        Ok(MysqlRepository { conn: "Test".to_string() })
    }
}
impl UserRepository for MysqlRepository {
    fn create_table(&self) -> Result<(), Error> {
        Ok(())
    }
    fn create_user(&self, _input: CreateUserInputDTO) -> Result<CreateUserOutputDTO, Error> {
        Ok(CreateUserOutputDTO {
            user: User {
                id: None,
                name: "test".to_string(),
            }
        })
    }
}

pub struct CreateUserInputDTO {
    pub name: String,
}

pub struct CreateUserOutputDTO {
    pub user: User,
}
