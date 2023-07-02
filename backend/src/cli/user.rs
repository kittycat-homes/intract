use std::error::Error;

use anyhow::anyhow;
use diesel::{Connection, ExpressionMethods, PgConnection, RunQueryDsl};

use crate::{config::CONFIG, db::models::Powerlevel, pass::hash_password_and_generate_salt};

pub fn change_powerlevel(usrname: &str, new_powerlevel: &Powerlevel) -> Result<(), Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    let mut conn = PgConnection::establish(&CONFIG.database.db_url)?;

    diesel::update(users)
        .filter(username.eq(usrname))
        .set(powerlevel.eq(new_powerlevel))
        .execute(&mut conn)?;

    tracing::info!("successfully set powerlevel of {} to {:#?}", usrname, new_powerlevel);
    Ok(())
}

pub fn change_password(usrname: &str, new_password: &str) -> Result<(), Box<dyn Error>> {
    use crate::schema::users::dsl::*;

    if new_password.len() < CONFIG.server.min_password_size as usize {
        return Err(anyhow!("password too short!"))?;
    }

    let pass_n_salt = hash_password_and_generate_salt(new_password)?;

    let mut conn: PgConnection = PgConnection::establish(&CONFIG.database.db_url)?;

    diesel::update(users)
        .filter(username.eq(usrname))
        .set((salt.eq(pass_n_salt.salt), hash.eq(pass_n_salt.hash)))
        .execute(&mut conn)?;

    tracing::info!("successfully updated password");
    Ok(())
}
