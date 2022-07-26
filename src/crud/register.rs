use crate::models::{UserInsertable,User};
use crate::diesel::RunQueryDsl;

use diesel::{PgConnection};

pub fn insert_new_user <'a>(
    nu: &'a UserInsertable,
    conn: &PgConnection
,
) -> Result<User, diesel::result::Error> {
    
    use crate::schema::users;

    let ret_val = diesel::insert_into(users::table).values(nu).get_result(conn);
    return ret_val;
}