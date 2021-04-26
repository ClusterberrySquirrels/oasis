extern crate tera;
// From what I can tell the macro is allowing the schema.rx file to actually
// generate in our binary. The extern I have no idea because from my
// understanding use supersedes it now. Using "use diesel::*" causes the macros
// in schema.rs to not get created properly.
#[macro_use]
extern crate diesel;

// We expose our models file and also include it.  Pub models will expose our
// models file.  The
pub mod schema;
pub mod models;

use actix_web::{get, post, HttpServer, App, web, HttpResponse, Responder, HttpRequest};
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use tera::{Tera, Context};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use models::{User, NewUser, LoginUser, Post, NewPost, Comment, NewComment};
use actix_web::error::PayloadError::Http2Payload;

#[derive(Deserialize)]
struct CommentForm {
    comment: String,
}

// This struct is the object that we want to serialize so that we can have Tera
// render the object without us moving all the data in the struct to the tera
// Context manually.  We add the derive statement to our struct and it will
// then be given automatic serialization.
#[derive(Serialize, Deserialize)]
struct PostForm {
    title: String,
    link: String,
    // author: String,
}

#[derive(Debug, Deserialize)]
struct Submission {
    title: String,
    link: String,
}

async fn comment(
    data: web::Form<CommentForm>,
    id: Identity,
    web::Path(post_id): web::Path<i32>
) -> impl Responder {

    if let Some(id) = id.identity() {
        use schema::posts::dsl::{posts};
        use schema::users::dsl::{users, username};

        let connection = establish_connection();

        let post :Post = posts.find(post_id)
            .get_result(&connection)
            .expect("Failed to find post.");

        let user :Result<User, diesel::result::Error> = users
            .filter(username.eq(id))
            .first(&connection);

        match user {
            Ok(u) => {
                let parent_id = None;
                let new_comment = NewComment::new(data.comment.clone(), post.id, u.id, parent_id);

                use schema::comments;
                diesel::insert_into(comments::table)
                    .values(&new_comment)
                    .get_result::<Comment>(&connection)
                    .expect("Error saving comment.");


                return HttpResponse::Ok().body("Commented.");
            }
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Ok().body("User not found.");
            }
        }
    }

    HttpResponse::Unauthorized().body("Not logged in.")
}

// Function to establish connection to database
// This is the connector that called anytime we want to connect to our databse
// and do something with it. We include some pars of diesel and we also include
// the dotenv crate. We start by making sure that our environment file is
// properly set up and then we read in the database url from the environment.
// The dotenv package would have done this.
// We then return a connection handler with the type PgConnection.
fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

// ** Function Index **
// We use the unwrap function because if tera fails for whatever reason, our
// entire application would be moot so panicking would be the best bet.
// Had we wanted to handle the error gracefully we could use unwrap_or_else
// or we can do the match construct.  In our case plain unwrap is fine.  Our
// index function should however error gracefully to know when something goes
// wrong.
// Our index function can access tera by passing it in via the function
// parameters with a type of web::Data.
// This index function starts off by building a key value object called data
// with the constructor being Context. We then run the renderer and its
// associated data.  All the tera.render is doing is processing the template
// along with the data to generate HTML. this HTML string is then sent back as
// a response to the browser.
// When a user visits the site, the index page will display a list of articles
// they can read and comment on.
// Inside the index, we created an array of posts that we can then loop through
// in our template file.
// We then pass data and the page we want to use to tera.render and we will
// then get an HTML string that we can then return to the user.
// The 'base.html' file contains block content meaning, that the child templates
// we can set up a block content that will then get placed in the parent template.
// The 'index.html' file extends the 'base.html' and creates our block "content".
// This way our templates will only hold what they need.
async fn index(tera: web::Data<Tera>) -> impl Responder {
    use schema::posts::dsl::{posts};
    use schema::users::dsl::{users};

    let connection = establish_connection();
    let all_posts: Vec<(Post, User)> = posts.inner_join(users)
        .load(&connection)
        .expect("Error retrieving all posts.");

    let mut data = Context::new();
    data.insert("title", "The Oasis");
    data.insert("posts_users", &all_posts);

    let rendered = tera.render("index.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// Users navigate to login page to create profile where they provide
// username, password and email to gain access to login.
// We use the unwrap function because if tera fails for whatever reason, our
// entire application would be moot so panicking would be the best bet.
// Had we wanted to handle the error gracefully we could use unwrap_or_else
// or we can do the match construct.  In our case plain unwrap is fine.
// The signup function is set up as a new route in the main where calls the
// signup function.  The signup function will then set the title and pass the
// data and the page we want to render to tera.render.
async fn signup(tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Sign Up");

    let rendered = tera.render("signup.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// This process inserts user information into SQL database using insert_into.
// We have rust print out what it receives on a HTTP POST to the sign up page.
// We added the route for the post request in the main and have this route run
// our process_signup function.
// We get the data out of our post request and we do this with the Form utility
// from actix::web.  This will let us get the data out of the request so that
// we can process it.
// We update our process_signup function to use the database connector and models.
// The Form extractor is set to NewUser
async fn process_signup(data: web::Form<NewUser>) -> impl Responder {
    // Here we are bringing the code that is generated through the macros in
    // the schema file. This will let us refer to the users table.
    use schema::users;

    // Create a database connection to do insertions on the database.
    let connection = establish_connection();

    // We do a validation on our new user such as making sure the username is
    // unique, the email is valid and the password is strong enough.
    // Duplicate users will cause our UNIQUE constraint we wrote in the sql
    // files to be violated and this will cause rust to panic.
    diesel::insert_into(users::table)
        .values(&*data)
        // This is where we execute our insert passing in the connection and
        // casting it to the type of User. The get_result call returns our
        // newly loaded item and we need to cast it properly.
        .get_result::<User>(&connection)
        // This triggers rust to panic if there are any errors in inserting.
        // Example: if we try to register an existing user we should see a panic.
        .expect("Error registering user.");

    println!("{:?}", data);
    HttpResponse::Ok().body(format!("Successfully saved user: {}", data.username))
}

// Users provide credentials provided from login page to login.
// We use the unwrap function because if tera fails for whatever reason, our
// entire application would be moot so panicking would be the best bet.
// Had we wanted to handle the error gracefully we could use unwrap_or_else
// or we can do the match construct.  In our case plain unwrap is fine.
async fn login(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Login");
    // Since the add id parameter list is passed in, the "if let Some(id)" lets
    // us quickly check the id.identity() function. This checks to see if the
    // session token we saved in the cookie exists in our session table.  If
    // it does, the check will pass and we don't need to display the login
    // page. If it doesn't exist, then we should allow the user to log in.
    if let Some(id) = id.identity() {
        return HttpResponse::Ok().body("Already logged in.");
    }
    let rendered = tera.render("login.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}

// This process checks the users credentials and verifies if user is authentic or not.
// If they are not then an HTTP response message is returned with "user does not exist".
// If an invalid password is given then an HTTP response is returned with "Password incorrect".
// We use the unwrap function because if tera fails for whatever reason, our
// entire application would be moot so panicking would be the best bet.
// Had we wanted to handle the error gracefully we could use unwrap_or_else
// or we can do the match construct.  In our case plain unwrap is fine.

async fn process_login(data: web::Form<LoginUser>, id: Identity) -> impl Responder {
    // We include our schema so we can use the user table.
    use schema::users::dsl::{username, users};

    // Create a connection to postgres by creating a new connection on each
    // request which can get expensive.  In the future we will set up a pool
    // of connections that we can then just use.
    let connection = establish_connection();
    // Check the database for the user by way of filter because we put a UNIQUE
    // constraint on our field when making our sql file, we can grab just the
    // first result we get. We pass in the connection to first which will
    // execute our filter.
    let user = users.filter(username.eq(&data.username)).first::<User>(&connection);
    // Now we will get a result type that we can match against.
    match user {
        // If the result was OK then we can go into the next set of logic
        Ok(u) => {
            // If the result was OK, then we can check the password, and if they
            // match we will print our original login message to our terminal.
            if u.password == data.password {
                // If the password check passes, we will create our session
                // token and add it to our session table. This also sets the
                // user's cookie with that information.
                let session_token = String::from(u.username);
                // What actix_identity's id option is doing is it's taking our
                // value and it creates a hash out of it that it keeps in it's
                // own table.
                // Wrapping the IdentityService around our app, this means thate
                // when a request comes in, it grabs the "auth-cookie" set in the
                // main and does a look up to see what the corresponding id should
                // be. This value is set in the .remember().
                id.remember(session_token);
                HttpResponse::Ok().body(format!("Logged in: {}", data.username))
                // If the passwords don't match, we'll let the user know.
            } else {
                HttpResponse::Ok().body("Password is incorrect.")
            }
        }
        // If the result is an Err, we will print a very helpful message.
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::Ok().body("User doesn't exist.")
        }
    }
}

// This process removes the identity and logs the user out of the session
async fn logout(id: Identity) -> impl Responder {
    // id.forget() removes the session token we set from the session table
    // and our cookie. To see how this works, you can log into the site and
    // then open the developer console and look under the storage tab. Here
    // you should see a cookie with a random string in it. Inside actix_identity
    // we have a table of these random strings matched to our real values, so
    // when we call id in our rust function, we will be able to get what we need.
    id.forget(); // remove identity
    HttpResponse::Ok().body("Logged out.")
}

// The first thing we need to do is register our route, but we can't use our
// trusty route option anymore as we're trying to also pass in data via the
// url. Now we can register a service which allows us to do more configuration
// on the route. This way we can have wildcards and dynamic variables in our
// paths and still process them.
async fn post_page(tera: web::Data<Tera>,
                   id: Identity,
                   web::Path(post_id): web::Path<i32>) -> impl Responder {
    use schema::posts::dsl::{posts};
    use schema::users::dsl::{users};

    let connection = establish_connection();

    // Were going to load the Post and User and display comments
    let post: Post = posts.find(post_id)
        .get_result(&connection)
        .expect("Failed to find post.");

    let user: User = users.find(post.author)
        .get_result(&connection)
        .expect("Failed to find user.");

    let comments :Vec<(Comment, User)> = Comment::belonging_to(&post)
        .inner_join(users)
        .load(&connection)
        .expect("Failed to find comments.");

    // We bring up the tables we need, then we set up a connection to our DB.
    let mut data = Context::new();
    data.insert("title", &format!("{} - The Oasis", post.title));
    data.insert("post", &post);
    data.insert("user", &user);
    data.insert("comments", &comments);

    if let Some(_id) = id.identity() {
        data.insert("logged_in", "true");
    } else {
        data.insert("logged_in", "false");
    }

    let rendered = tera.render("post.html", &data).unwrap();
    HttpResponse::Ok().body(rendered)
}


// This function is provided for users to post messages to the site page.
async fn submission(tera: web::Data<Tera>, id: Identity) -> impl Responder {
    let mut data = Context::new();
    data.insert("title", "Submit a Post");

    // We will check the id and if the user is logged in, we will let them
    // access the submission page.
    if let Some(id) = id.identity() {
        let rendered = tera.render("submission.html", &data).unwrap();
        return HttpResponse::Ok().body(rendered);
    }
    // If the user isn't logged in, return an unauthorized response.
    HttpResponse::Unauthorized().body("401 - Unauthorized response: \n User not logged in.")
}

// Here the form is updated where PostForm extractor and id are passed in as
// parameters. This will do the checking to make sure that the submission is
// coming from a logged in user.
async fn process_submission(data: web::Form<PostForm>, id: Identity) -> impl Responder {
    if let Some(id) = id.identity() {
        use schema::users::dsl::{username, users};

        let connection = establish_connection();
        // Once the session has been confirmed that is valid we bring in the
        // domain specific language for the users table. This will allow us
        // to figure out who the user is.
        let user: Result<User, diesel::result::Error> = users.filter(username.eq(id)).first(&connection);
        // In this case, the username is the token in the username so we can
        // reverse it to a user id easily by querying the user table. If our
        // token been a random string that we kept matched to the user, we
        // would need to first go to that table to get the user id.


        match user {
            Ok(u) => {
                // Once we have the User we make sure we have a valid result
                // and then we convert our PostForm to a NewPost. this line
                // admittedly does bother me as we are doing a clone to pass
                // the data. I did not figure out what the borrowing rules
                // here should be.
                let new_post = NewPost::from_post_form(data.title.clone(), data.link.clone(), u.id);
                // The next step is to bring in the posts table which we do
                // use schema::posts line.
                use schema::posts;

                // Next we insert our NewPost object into our posts table, reusing
                // the connection we setup earlier in our function.
                diesel::insert_into(posts::table)
                    .values(&new_post)
                    .get_result::<Post>(&connection)
                    .expect("Error saving post.");

                return HttpResponse::Ok().body("Submitted.");
            }
            Err(e) => {
                println!("{:?}", e);
                return HttpResponse::Ok().body("Failed to find user.");
            }
        }
    }
    HttpResponse::Unauthorized().body("User not logged in.")
}

// Includes the use of actix_web and then starts the server with HttpServer::new().run()
// Server is set to respond with 200 OK
// Server is bound to the localhost IP address and will respond to anything that comes
// in on the port assigned.
// We use the unwrap function because if tera fails for whatever reason, our
// entire application would be moot so panicking would be the best bet.
// Had we wanted to handle the error gracefully we could use unwrap_or_else
// or we can do the match construct.  In our case plain unwrap is fine.
// We register the tera object into our App with the use of the .data method.
// This way any functions we run in our App will always have access to tera.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        // With Tera, our templating engine, we wanted to make a variable
        // accessible to functions we call within our App.
        let tera = Tera::new("templates/**/*").unwrap();
        // Here we register our IdentityService with the wrap option, on our App
        // object which sits inside our HttpServer. We will now have the
        // ability to create sessions.
        App::new()
            .data(tera)
            // We register IdentityService in our app, similar to how we did with tera.
            .wrap(IdentityService::new(
                // We make sure the requests coming in have a cookie set on them
                // and that we are always sending that information to the user
                // and the browser on the other side will make sure we get that
                // information back.  We want the ID service so instead of using
                // .data() to register, we will use .wrap().
                CookieIdentityPolicy::new(&[0; 32])
                    // This means that when a request comes in, it grabs the
                    // "auth-cookie" and does a look up to see what the
                    // corresponding id should be. This is the value set inside
                    // the .remember() function in the process_login function.
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
            .service(
                web::resource("/post/{post_id}")
                    .route(web::get().to(post_page))
                    .route(web::post().to(comment))
            )
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
    use actix_web::{test, web, App};

    // Default test
    #[test]
    fn test_index() {
        let result = Data::new(Default::default());
        index(result);
        // assert!("This test is the default!")
    }

    // Methods to used to send request to server using TestRequest::get() and
    // TestRequest::post where a service is created for testing, using the
    // test::init_service method to accept a regular App builder.

    // Test index
    #[actix_rt::test]
    async fn test_index_get() {
        let tera = Tera::new("templates/**/*").unwrap();
        let mut app = test::init_service(
            App::new().data(tera).route("/", web::get().to(index))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        print!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    // Test login
    #[actix_rt::test]
    async fn test_login_get() {
        let tera = Tera::new("templates/**/*").unwrap();
        let mut app = test::init_service(
            App::new().data(tera).route("/", web::get().to(login))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        print!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    // Test logout
    #[actix_rt::test]
    async fn test_logout_get() {
        let tera = Tera::new("templates/**/*").unwrap();
        let mut app = test::init_service(
            App::new().data(tera).route("/", web::get().to(logout))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        print!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    // Test signup
    #[actix_rt::test]
    async fn test_signup_get() {
        let tera = Tera::new("templates/**/*").unwrap();
        let mut app = test::init_service(
            App::new().data(tera).route("/", web::get().to(signup))).await;
        let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
        let resp = test::call_service(&mut app, req).await;
        print!("{}", resp.status());
        assert!(resp.status().is_success());
    }

    // FIXME: not passing
    // // Test submission
    // #[actix_rt::test]
    // async fn test_submission_get() {
    //     let tera = Tera::new("templates/**/*").unwrap();
    //     let mut app = test::init_service(
    //         App::new().data(tera).route("/", web::get().to(submission))).await;
    //     let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    //     let resp = test::call_service(&mut app, req).await;
    //     print!("{}", resp.status());
    //     assert!(resp.status().is_success());
    // }

    // Test panic
    // #[test]
    //     fn another() {
    //
    //     panic!("Make this test fail!")
    // }
}