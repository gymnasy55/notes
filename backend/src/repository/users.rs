use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::model::user::User;
use crate::repository::establish_connection;
use crate::schema::users::dsl::*;

pub struct UsersRepository {
    connection: PgConnection,
}

#[derive(Debug)]
pub struct UsersError;

impl UsersRepository {
    pub fn init() -> UsersRepository {
        UsersRepository {
            connection: establish_connection(),
        }
    }

    pub fn get_users(&self) -> Result<Vec<User>, UsersError> {
        let req = users.load::<User>(&self.connection);

        match req {
            Ok(res) => Ok(res),
            Err(_) => Err(UsersError),
        }
    }

    pub fn get_user_by_id(&self, user_id: String) -> Result<Option<User>, UsersError> {
        let req = users.filter(id.eq(user_id)).load::<User>(&self.connection);

        match req {
            Ok(res) => match res.first() {
                Some(v) => Ok(Some(v.clone())),
                None => Ok(None),
            },
            Err(_) => Err(UsersError),
        }
    }

    pub fn insert_user(&self, user: User) -> Result<(), UsersError> {
        let req = diesel::insert_into(users)
            .values(user)
            .get_result::<User>(&self.connection);

        match req {
            Ok(_) => Ok(()),
            Err(_) => Err(UsersError),
        }
    }

    pub fn delete_user(&self, user_id: String) -> Result<(), UsersError> {
        let req = diesel::delete(users.filter(id.eq(&user_id))).execute(&self.connection);

        match req {
            Ok(result) => match result {
                0 => Err(UsersError),
                _ => Ok(()),
            },
            Err(_) => Err(UsersError),
        }
    }
}
