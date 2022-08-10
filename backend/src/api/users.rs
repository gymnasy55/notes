use serde::{Deserialize, Serialize};

use crate::{api::error::users::UserError, repository::users::UsersRepository};
use actix_web::{
    get,
    web::{Data, Json, Path},
};

#[derive(Deserialize, Serialize)]
pub struct UserId {
    pub id: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct GetUserResponse {
    pub id: String,
    pub email: String,
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug)]
pub struct GetUsersResponse {
    pub users: Vec<GetUserResponse>,
}

#[get("/{id}")]
pub async fn get_user(
    user_repo: Data<UsersRepository>,
    user_id: Path<UserId>,
) -> Result<Json<GetUserResponse>, UserError> {
    let res = user_repo.get_user_by_id(user_id.into_inner().id);

    match res {
        Ok(option) => match option {
            Some(user) => Ok(Json(GetUserResponse {
                id: user.id,
                email: user.email,
            })),
            None => Err(UserError::NotFound),
        },
        Err(_) => Err(UserError::InternalError),
    }
}

#[get("")]
pub async fn get_users(
    user_repo: Data<UsersRepository>,
) -> Result<Json<GetUsersResponse>, UserError> {
    let res = user_repo.get_users();

    match res {
        Ok(users) => Ok(Json(GetUsersResponse {
            users: users
                .iter()
                .map(|user| {
                    let res = user.clone();
                    GetUserResponse {
                        id: res.id,
                        email: res.email,
                    }
                })
                .collect::<Vec<_>>(),
        })),
        Err(_) => Err(UserError::InternalError),
    }
}

#[cfg(test)]
mod tests {
    use crate::api::users::GetUsersResponse;
    use crate::{
        api::users::{get_user, get_users, GetUserResponse},
        helpers::defer::{defer, expr, ScopeCall},
        model::user::User,
        repository::users::UsersRepository,
    };
    use actix_web::{
        body,
        body::MessageBody,
        dev::{ServiceFactory, ServiceRequest, ServiceResponse},
        error, test,
        web::{scope, Bytes, Data},
        App,
    };
    use rand::*;
    use rand_regex::*;
    use rand_xorshift::*;
    use regex_syntax::*;
    use ring::rand::{SecureRandom, SystemRandom};

    fn gen_emails(number: usize) -> Vec<String> {
        fn seed() -> [u8; 16] {
            let mut salt: [u8; 16] = [0u8; 16];
            match SystemRandom::new().fill(&mut salt) {
                Ok(()) => salt,
                Err(e) => panic!("{:?}", e),
            }
        }

        let mut rng = XorShiftRng::from_seed(seed());
        let mut parser = ParserBuilder::new().unicode(false).build();
        let hir = match parser.parse(r"[a-z]{10}@gmail\.com") {
            Ok(hir) => hir,
            Err(e) => panic!("{:?}", e),
        };
        let gen = Regex::with_hir(hir, 100).unwrap();

        (&mut rng)
            .sample_iter(&gen)
            .take(number)
            .collect::<Vec<String>>()
    }

    fn app(
        repo: &Data<UsersRepository>,
    ) -> App<
        impl ServiceFactory<
            ServiceRequest,
            Response = ServiceResponse<impl MessageBody>,
            Config = (),
            InitError = (),
            Error = error::Error,
        >,
    > {
        App::new()
            .app_data(repo.clone())
            .service(scope("/users").service(get_user).service(get_users))
    }

    fn insert_user(user: &User, repo: &UsersRepository) {
        match repo.insert_user(user.clone()) {
            Ok(()) => (),
            Err(e) => panic!("{}", e),
        }
    }

    fn delete_user(user: &User, repo: &UsersRepository) {
        match repo.delete_user(user.clone().id) {
            Ok(()) => (),
            Err(_) => panic!("Unable to delete user"),
        }
    }

    #[actix_web::test]
    async fn get_user_should_return_correct() {
        let user = User::new(
            gen_emails(1).first().unwrap().clone(),
            String::from("test"),
            None,
        );
        let repo = Data::new(UsersRepository::init());
        insert_user(&user, &repo);
        defer!(delete_user(&user, &repo));

        let app = test::init_service(app(&repo)).await;
        let req = test::TestRequest::get()
            .uri(format!("/users/{}", user.id).as_str())
            .to_request();
        let res: GetUserResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(
            res,
            GetUserResponse {
                id: user.id.clone(),
                email: user.email.clone(),
            }
        );
    }

    #[actix_web::test]
    async fn get_user_should_return_not_found() {
        let email = gen_emails(1).first().unwrap().clone();
        let repo = Data::new(UsersRepository::init());

        let app = test::init_service(app(&repo)).await;
        let req = test::TestRequest::get()
            .uri(format!("/users/{}", email).as_str())
            .to_request();
        let res = test::call_service(&app, req).await;
        let status = res.status().as_u16();
        let body = match body::to_bytes(res.into_body()).await {
            Ok(body) => body,
            Err(_) => panic!("Unable to parse body"),
        };

        assert_eq!(status, 404);
        assert_eq!(body, Bytes::from_static(b"NotFound"))
    }

    #[actix_web::test]
    async fn get_users_should_return_users() {
        let emails = gen_emails(2);
        let users = vec![
            User::new(emails.get(0).unwrap().clone(), String::from("test"), None),
            User::new(emails.get(1).unwrap().clone(), String::from("test"), None),
        ];
        let repo = Data::new(UsersRepository::init());
        let _ = users
            .iter()
            .map(|user| insert_user(user, &repo))
            .collect::<Vec<_>>();
        defer! {
            let _ = users.iter().map(|user| delete_user(user, &repo)).collect::<Vec<_>>();
        }

        let app = test::init_service(app(&repo)).await;
        let req = test::TestRequest::get().uri("/users").to_request();
        let res: GetUsersResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(
            res,
            GetUsersResponse {
                users: users
                    .iter()
                    .map(|user| {
                        GetUserResponse {
                            id: user.id.clone(),
                            email: user.email.clone(),
                        }
                    })
                    .collect::<Vec<_>>()
            }
        )
    }

    #[actix_web::test]
    async fn get_users_should_return_empty() {
        let repo = Data::new(UsersRepository::init());

        let app = test::init_service(app(&repo)).await;
        let req = test::TestRequest::get().uri("/users").to_request();
        let res: GetUsersResponse = test::call_and_read_body_json(&app, req).await;

        assert_eq!(res, GetUsersResponse { users: vec!() })
    }
}
