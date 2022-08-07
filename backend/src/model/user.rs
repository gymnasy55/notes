use diesel::Insertable;
use uuid::Uuid;

use crate::helpers::password;
use crate::password::CREDENTIAL_SIZE;
use crate::schema::users;

pub trait UserNewWithId {
    fn new(
        id: String,
        email: String,
        password: String,
        salt: Option<[u8; CREDENTIAL_SIZE]>,
    ) -> User;
}

#[derive(Queryable, Insertable, PartialEq, Debug, Clone)]
#[table_name = "users"]
pub struct User {
    pub id: String,
    pub email: String,
    pub encrypted_password: String,
    pub salt: String,
}

impl UserNewWithId for User {
    fn new(
        id: String,
        email: String,
        password: String,
        salt: Option<[u8; CREDENTIAL_SIZE]>,
    ) -> User {
        let hash_salt = match salt {
            Some(salt) => password::encrypt_with_salt(password, salt),
            None => password::encrypt(password),
        };

        User {
            id,
            email,
            encrypted_password: hash_salt.hash,
            salt: hash_salt.salt,
        }
    }
}

impl User {
    pub fn new(email: String, password: String, salt: Option<[u8; CREDENTIAL_SIZE]>) -> User {
        let hash_salt = match salt {
            Some(salt) => password::encrypt_with_salt(password, salt),
            None => password::encrypt(password),
        };

        User {
            id: Uuid::new_v4().to_string(),
            email,
            encrypted_password: hash_salt.hash,
            salt: hash_salt.salt,
        }
    }
}

#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::password::CREDENTIAL_SIZE;
    use crate::{User, UserNewWithId};

    const EMAIL: &str = "test@test.com";
    const PASSWORD: &str = "test";
    const SALT: [u8; CREDENTIAL_SIZE] = [
        249, 186, 110, 86, 188, 130, 204, 148, 153, 54, 33, 213, 158, 143, 216, 89, 22, 38, 101,
        72, 216, 197, 242, 126, 39, 71, 91, 108, 87, 220, 228, 53,
    ];

    #[test]
    fn user_new_id_should_return_correct() {
        let id = Uuid::new_v4().to_string();

        let expected = User {
            id: id.clone(),
            email: String::from(EMAIL),
            encrypted_password: String::from("shZHGkXULBZvk4Yvo0JUHtgSjUMPm1hKlRObZDBOfdQ="),
            salt: base64::encode(SALT),
        };
        let actual = <User as UserNewWithId>::new(
            id,
            String::from(EMAIL),
            String::from(PASSWORD),
            Some(SALT),
        );

        assert_eq!(actual, expected)
    }

    #[test]
    fn user_new_should_return_correct() {
        let expected = User {
            id: String::default(),
            email: String::from(EMAIL),
            encrypted_password: String::from("shZHGkXULBZvk4Yvo0JUHtgSjUMPm1hKlRObZDBOfdQ="),
            salt: base64::encode(SALT),
        };
        let actual = User::new(String::from(EMAIL), String::from(PASSWORD), Some(SALT));

        assert_eq!(actual.email, expected.email);
        assert_eq!(actual.encrypted_password, expected.encrypted_password);
        assert_eq!(actual.salt, expected.salt);
    }
}
