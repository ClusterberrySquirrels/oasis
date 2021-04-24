// We use the schema.rs file via the super option because the models.rs file is
// under the root, main.rs file.
use super::schema::users;
use diesel::{Queryable, Insertable};
use serde::Deserialize;

// We are exposing our structs to other parts of our application through the pub
// keyword.  We can also keep things private if we need to.

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
#[derive(Queryable, Debug)]
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

// This also derives Debug so we can print the data out.
#[derive(Deserialize, Debug)]
pub(crate) struct LoginUser {
    pub username: String,
    pub password: String,
}