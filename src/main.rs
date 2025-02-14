use actix_web::{ get, web, App, HttpServer, Responder, HttpResponse, Result };

pub mod secret_utils;
pub mod crypt_utils;
pub mod constant;
pub mod file_utils;
pub mod api;
pub mod xor_shift_encrypt_utils;
pub mod file_binary_encrypt_utils;

const PORT: u16 = 8081;
const HOST: &str = "127.0.0.1";
const OK_4201: &str = "4201";

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/keysafer/secret")]
async fn secret() -> Result<impl Responder> {
    Ok(api::get_secret())
}

#[get("/keysafer/password")]
async fn password() -> Result<impl Responder> {
    Ok(api::get_password())
}

#[get("/keysafer/password/encrypt")]
async fn password_encrypt_api() -> impl Responder {
    api::password_encrypt();
    HttpResponse::Ok().body(OK_4201)
}

#[get("/keysafer/password/recover")]
async fn password_recover_api() -> impl Responder {
    api::password_recover();
    HttpResponse::Ok().body(OK_4201)
}

#[get("/keysafer/mnemonic/encrypt")]
async fn mnemonic_encrypt_api() -> impl Responder {
    api::mnemonic_encrypt();
    HttpResponse::Ok().body(OK_4201)
}

#[get("/keysafer/mnemonic/recover")]
async fn mnemonic_recover_api() -> impl Responder {
    api::mnemonic_recover();
    HttpResponse::Ok().body(OK_4201)
}

#[get("/keysafer/mnemonic/get/{id}")]
async fn mnemonic_get_api(path: web::Path<(String,)>) -> impl Responder {
    let id = path.into_inner().0;
    HttpResponse::Ok().body(api::get_mnemonic(&id))
}

#[get("/keysafer/encrypt/{text}")]
async fn encrypt(path: web::Path<(String,)>) -> impl Responder {
    let text = path.into_inner().0;
    HttpResponse::Ok().body(api::encrypt(&text))
}

#[get("/keysafer/file/encrypt")]
async fn file_encrypt_api() -> impl Responder {
    api::file_encrypt();
    HttpResponse::Ok().body(OK_4201)
}

#[get("/keysafer/file/recover")]
async fn file_recover_api() -> impl Responder {
    api::file_recover();
    HttpResponse::Ok().body(OK_4201)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(secret)
            .service(password)
            .service(encrypt)
            .service(password_recover_api)
            .service(password_encrypt_api)
            .service(mnemonic_recover_api)
            .service(mnemonic_encrypt_api)
            .service(mnemonic_get_api)
            .service(file_recover_api)
            .service(file_encrypt_api)
    })
        .bind((HOST, PORT))?
        .run().await
}

// fn main() {
//     // api::password_encrypt();
//     // api::password_recover();
//     // api::mnemonic_encrypt();
//     // api::mnemonic_recover();
//     // api::get_mnemonic("aas");
//     // api::get_password()

//     // api::file_encrypt();
//     api::file_recover();
// }
