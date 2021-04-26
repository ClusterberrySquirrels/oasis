// We use the schema.rs file via the super option because the models.rs file is
// under the root, main.rs file.
use super::schema::{users, posts,comments};
use diesel::{Queryable, Insertable};
use serde::{Serialize,Deserialize};
use crate::dotenv;
use argonautica::Hasher;
// We are exposing our structs to other parts of our application through the pub
// keyword.  We can also keep things private if we need to.

#[derive(Debug, Serialize, Queryable, Identifiable, Associations)]
#[belongs_to(Post)]
pub struct Comment {
    pub id: i32,
    pub comment: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize, Insertable)]
#[table_name="comments"]
pub struct NewComment {
    pub comment: String,
    pub post_id: i32,
    pub user_id: i32,
    pub parent_comment_id: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
}

impl NewComment {
    pub fn new(comment: String, post_id: i32,
               user_id: i32, parent_comment_id: Option<i32>) -> Self{
        NewComment {
            comment: comment,
            post_id: post_id,
            user_id: user_id,
            parent_comment_id: parent_comment_id,
            created_at: chrono::Local::now().naive_utc(),
        }
    }
}

// To extract the data we need to be able to take the string
// data in the post request and convert that into a rust object.  For this we
// want to do the opposite of serialize.  This also derives Debug so
// we can print the data out.
// The User struct corresponds to existing users, it's almost like extracting
// a full user from the database.  This user will have the extra parameter of
// id. This struct has the Queryable trait because we want to be able to query
// the User table and get everything structure using the User struct.
// Note: Because the struct has the Queryable trait, we need to make sure the
// order and types match what is in the schema.
#[derive(Serialize, Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
}

// To extract the data we need to be able to take the string
// data in the post request and convert that into a rust object.  For this we
// want to do the opposite of serialize.
// We include the Deserialize utility from serde and then we derive it for our
// User struct.  This struct matches the form we have in our template.  This
// also derives Debug so we can print the data out.
// We need two versions of user when we interact with the User table.
// The NewUser struct corresponds to a user that we will extract from a
// request and will put into the User table.  This will derive the Insertable
// trait.
#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl NewUser {
    pub fn new(username: String, email: String, password: String) -> Self {
        dotenv().ok();

        let secret = std::env::var("SECRET_KEY")
            .expect("SECRET_KEY must be set");

        let hash = Hasher::default()
            .with_password(password)
            .with_secret_key(secret)
            .hash()
            .unwrap();

        NewUser {
            username: username,
            email: email,
            password: hash,
        }
    }
}

// This also derives Debug so we can print the data out.
#[derive(Deserialize, Debug)]
pub(crate) struct LoginUser {
    pub username: String,
    pub password: String,
}

// The only strange field is the link as a post could just be a title. We
// signified this in our SQL file by not giving the "NOT NULL" condition.
// In our schema file it appears a Nullable and in our struct it should be
// Option.
// The other thing to note is that our create_at is a type from the
// chrono crate. These types aren't included in serde so if we didn't
// enable serde in our chrono crate we would have issues with the
// Serialization and Deserialization traits.
#[derive(Serialize, Debug, Queryable, Identifiable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub link: Option<String>,
    pub author: i32,
    pub created_at: chrono::NaiveDateTime,
}

// NewPost struct contains all the fields we want to set when we go to insert
// into our posts table. The two extra fields here are author and created_at
// both of which we will not extra from the form. This is why we need a third.
// struct. What we will do is convert our existing PostForm to a NewPost and
// then insert that into our table.  To do this we will implement in the
// method NewPost.
#[derive(Deserialize, Insertable)]
#[table_name="posts"]
pub struct NewPost {
    pub title: String,
    pub link: String,
    pub author: i32,
    pub created_at: chrono::NaiveDateTime,
}

// This creates a function that will build a NewPost object from a title, link
// and user id we pass in.
impl NewPost{
    pub fn from_post_form(title: String, link: String, uid: i32) -> Self {
        NewPost {
            title: title,
            link: link,
            author: uid,
            created_at: chrono::Local::now().naive_utc(),
        }
    }
}