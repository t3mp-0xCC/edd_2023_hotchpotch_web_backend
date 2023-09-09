use anyhow::{anyhow, Context};
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;
use uuid::uuid;

use crate::db::establish_connection;
use crate::models::{self, Event, Join, Request, User, Solo, Team, NewEvent, NewJoin, NewRequest, NewUser, NewSolo, NewTeam};
use crate::models::Hoge;
use crate::schema::*;

// Create
pub fn create_event (
    name: &String,
    desc: &String,
    url:  &String
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let new_event = NewEvent{name, desc, url};
    insert_into(events::dsl::events)
        .values(&new_event)
        .execute(conn)
        .with_context(|| "Failed to insert new_event")?;
    Ok(())
}

pub fn create_join (
    team_id: &String,
    user_id: &String
) -> anyhow::Result<()> {
    let team_id = match &conv_string_to_uuid(team_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let user_id = match &conv_string_to_uuid(user_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let conn = &mut establish_connection()?;
    let new_join = NewJoin{team_id, user_id};
    insert_into(joins::dsl::joins)
        .values(&new_join)
        .execute(conn)
        .with_context(|| "Failed to insert new_join")?;
    Ok(())
}

pub fn create_request (
    team_id: &String,
    user_id: &String,
    message: &String
) -> anyhow::Result<()> {
    let team_id = match &conv_string_to_uuid(team_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let user_id = match &conv_string_to_uuid(user_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
  
    let conn = &mut establish_connection()?;
    let new_request = NewRequest{team_id, user_id, message};
    insert_into(requests::dsl::requests)
        .values(&new_request)
        .execute(conn)
        .with_context(|| "Failed to insert new_request")?;
    Ok(())
}

pub fn create_user (
    name: &String,
    icon_url: &String,
    profile: &String,
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let new_user = NewUser{name, icon_url, profile};
    insert_into(users::dsl::users)
        .values(&new_user)
        .execute(conn)
        .with_context(|| "Failed to insert new_user")?;
    Ok(())
}

pub fn create_solo (
    event_id: &String,
    user_id: &String
) -> anyhow::Result<()> {
    let event_id = match &conv_string_to_uuid(event_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let user_id = match &conv_string_to_uuid(user_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let conn = &mut establish_connection()?;
    let new_solo = NewSolo{event_id, user_id};
    insert_into(solos::dsl::solos)
        .values(&new_solo)
        .execute(conn)
        .with_context(|| "Failed to insert new_solo")?;
    Ok(())
}

pub fn create_team (
    event_id: &String,
    reader_id: &String,
    name: &String,
    desc: &String
) -> anyhow::Result<()> {
    let event_id = match &conv_string_to_uuid(event_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let reader_id = match &conv_string_to_uuid(reader_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let conn = &mut establish_connection()?;
    let new_team = NewTeam{event_id, reader_id, name, desc};
    insert_into(teams::dsl::teams)
        .values(&new_team)
        .execute(conn)
        .with_context(|| "Failed to insert new_team")?;
  
    Ok(())
}

// Read

// TODO: fix .load error
pub fn get_event_list() -> Vec<Event> {
    use crate::schema::events::dsl::events;
    let conn = &mut establish_connection().unwrap();
    events.select(Event::as_select()).first(conn).expect("Error getting new user")
}

pub fn get_hoge_list() -> Hoge {
    use crate::schema::hoge::dsl::hoge;
    let conn = &mut establish_connection().unwrap();
    hoge.select(Hoge::as_select()).first(conn).expect("Error getting new user")
} 
// TODO: impl
/* pub fn get_requests_from_user_id(user_id: &String) -> anyhow::Result<Vec<Request>> {
    let conn = &mut establish_connection()?;
    let user_id: Uuid = match Uuid::parse_str(user_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    let result = match requests::dsl::requests.load::<Request>(conn) {
        Ok(r) => r,
        Err(e) => return Err(anyhow!("{}", e)),
    };
} 
*/
// TODO: impl
/* pub fn get_wanna_join_users_by_event_id() -> anyhow::Result<Vec<User>> {
    let conn = establish_connection()?;
} */
 
// Update

// Delete
pub fn delete_event_by_id(event_id: &Uuid) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let target = events::dsl::events
        .filter(events::dsl::id.eq(event_id));
    diesel::delete(target)
        .execute(conn)
        .with_context(|| "Failed to delete event")?;
    Ok(())
}

// Utils
fn conv_string_to_uuid(str_uuid: &str) -> anyhow::Result<(Uuid)> {
    match Uuid::parse_str(str_uuid) {
        Ok(uuid) => return Ok(uuid),
        Err(e) => return Err(anyhow!("{}", e)),
    };
}
