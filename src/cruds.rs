use anyhow::{anyhow, Context};
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;
use uuid::uuid;

use crate::db::establish_connection;
use crate::models::{self, Event, Join, Request, User, Solo, Team, NewEvent, NewJoin, NewRequest, NewUser, NewSolo, NewTeam};
use crate::schema::{events, joins, requests, users, solos, teams};

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
    team_id: &Uuid,
    user_id: &Uuid
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let new_join = NewJoin{team_id, user_id};
    insert_into(joins::dsl::joins)
        .values(&new_join)
        .execute(conn)
        .with_context(|| "Failed to insert new_join")?;
    Ok(())
}

pub fn create_request (
    team_id: &Uuid,
    user_id: &Uuid,
    message: &String
) -> anyhow::Result<()> {
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
    event_id: &Uuid,
    user_id: &Uuid
) -> anyhow::Result<()> {
    let conn = &mut establish_connection()?;
    let new_solo = NewSolo{event_id, user_id};
    insert_into(solos::dsl::solos)
        .values(&new_solo)
        .execute(conn)
        .with_context(|| "Failed to insert new_solo")?;
    Ok(())
}

pub fn create_team (
    event_id: &Uuid,
    reader_id: &Uuid,
    name: &String,
    desc: &String
) -> anyhow::Result<()> {
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
/* pub fn get_event_list() -> anyhow::Result<Vec<Event>> {
    let conn = &mut establish_connection()?;
    let result = schema::events::dsl::events.select(Event::as_select()).load::<Event>(conn);
} */

// TODO: impl
/* pub fn get_requests_from_user_id(id: &String) -> anyhow::Result<Vec<Request>> {
    use crate::schema::requests::dsl::*;
    let conn = &mut establish_connection()?;
    let uuid: Uuid = match Uuid::parse_str(id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    match requests.filter(user_id.eq(uuid)).load::<Request>(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e))
    }
} */

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
