extern crate tera;
#[macro_use]
extern crate diesel;

pub mod schema;
pub mod models;

use actix_web::{get, post, HttpServer, App, web, HttpResponse, Responder, HttpRequest};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{User, NewUser, LoginUser};

#[derive(Serialize)]
struct Post {
    title: String,
    link: String,
    author: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

// Function to establish connection to database
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// Main navigation page to other links in the Oasis
async fn index(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();

    let posts = [
        Post {
            title: String::from("This is the first link"),
            link: String::from("https://example.com"),
            author: String::from("Nutrition-Tracker"),
        },
        Post {
            title: String::from("This is the second Link"),
            link: String::from("https://example.com"),
            author: String::from("Other cool app"),
        },
        Post {
            title: String::from("Logout"),
            link: String::from("/logout"),
            author: String::from("Logout from here"),
        }
    ];

    data.insert("title", "index");
    data.insert("posts", &posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// Users navigate to login page to create profile where they provide
// username, password and email to gain access to login.
async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// This process inserts user information into SQL database using the insert
async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    use schema::users;

    // Connect to database singleton object
    let connection = establish_connection();

    // Insert provided user information into user database
    diesel::insert_into(users::table)
        .values(&*data)
        .get_result::<User>(&connection)
        .expect("Error registering user."); // Error message if user doesn't enter all info

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

// Users provide credentials provided from login page to login.
async fn login(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");

    if let Some(id) = id.identity() {
        return HttpResponse::Ok().body("Already logged in.");
    }
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// This process removes the identity and logs the user out of the session
async fn logout(id: Identity) -> impl Responder {
    id.forget(); // remove identity
    HttpResponse::Ok().body("Logged out.")
}

// This process checks the users credentials and verifies if user is authentic or not.
// If they are not then an HTTP response message is returned with "user does not exist".
// If an invalid password is given then an HTTP response is returned with "Password incorrect".
async fn process_login(data: web::Form<LoginUser>, id: Identity) -> impl Responder {
    use schema::users::dsl::{username, users};

    let connection = establish_connection();
    let user = users.filter(username.eq(&data.username)).first::<User>(&connection);

    match user {
        Ok(u) => {
            if u.password == data.password {
                let session_token = String::from(u.username);
                id.remember(session_token);
                HttpResponse::Ok().body(format!("Logged in: {}", data.username))
            } else {
                HttpResponse::Ok().body("Password is incorrect.")
            }
        }
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::Ok().body("User doesn't exist.")
        }
    }
}

// This function is provided for users to post messages to the site page.
async fn submission(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    if let Some(id) = id.identity() {
        let rendered = tera.render("submission.html", &data).unwrap();
        HttpResponse::Ok().body(rendered);
    }
    HttpResponse::Unauthorized().body("401 - Unauthorized response: \n User not logged in.")
}

// Function procces for posting a submission.
async fn process_submission(data: web::Form<Submission>) -> impl Responder {
    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Posted submission: {}", data.title))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new("templates/**/*").unwrap();
        App::new()
            .data(tera)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("auth-cookie")
                    .secure(false)
            )
            )
            .route("/", web::get().to(index))
            .route("/signup", web::get().to(signup))
            .route("/signup", web::post().to(process_signup))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(process_login))
            .route("/logout", web::to(logout))
            .route("/submission", web::get().to(submission))
            .route("/submission", web::post().to(process_submission))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

// tests
#[cfg(test)]
mod tests {
   use super::*;
    use actix_web::web::Data;
    use actix_web::Responder;
    use actix_web::{test,http};

    #[test]
    fn test_index() {
        let result = Data::new(Default::default());
        index(result);

        // assert!(render);
    }

    #[actix_rt::test]
    async fn test_index_ok() {
        let result = Data::new(Default::default());
        let req = test::TestRequest::with_header("content-type", "text/plain").to_http_request();
        let resp = index(result).await;
        assert_eq!(resp.status(), http::StatusCode::OK);
    }
    // #[test]
    //     fn another() {
    //
    //     panic!("Make this test fail!")
    // }

}