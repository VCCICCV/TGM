// use application::use_case::register_user_use_case::RegisterUserUseCase;
// use axum::{response::IntoResponse, Json};
// use domain::model::dto::user_dto::RegisterUserSchema;
// use infrastructure::persistence::{
//     auth_repository_impl::AuthServiceImpl, user_repository_impl::UserRepositoryImpl,
// };

// use crate::common::response::Res;

// pub async fn register_handler(user: Json<RegisterUserSchema>) -> impl IntoResponse {
//     let register_user_use_case = RegisterUserUseCase::new(
//         UserRepositoryImpl {},
//         AuthServiceImpl::new(UserRepositoryImpl {}),
//     );
//     match register_user_use_case.register(&user.0).await {
//         Ok(token) => Res::with_data(token),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }
// pub async fn login_handler(user: Json<LoginUserSchema>) -> impl IntoResponse {
//     let login_user_use_case = LoginUserUserCase::new(AuthServiceImpl::new(UserRepositoryImpl {}));
//     match login_user_use_case.login(user.0).await {
//         Ok(token) => Res::with_data(token),
//         Err(e) => Res::with_err(&e.to_string()),
//     }
// }
// // pub async fn refresh_token_handler(Json(refresh_token): Json<String>) -> impl IntoResponse {
// //     match refresh_token_use_case.refresh(&refresh_token).await {
// //         Ok((token, refresh_token)) => Res::with_data((token, refresh_token)),
// //         Err(e) => Res::with_err(&e.to_string()),
// //     }
// // }

// // pub async fn authenticate_handler(user: Json<User>) -> impl IntoResponse {
// //     let user_use_case = UserUseCase::new();
// //     match user_use_case.authenticate(user.0).await {
// //         Ok(_) => Res::with_data("ok"),
// //         Err(e) => Res::with_err(&e.to_string()),
// //     }
// // }
