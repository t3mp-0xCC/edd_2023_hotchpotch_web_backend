use anyhow::{anyhow, Context};
use diesel::dsl::any;
use diesel::insert_into;
use diesel::prelude::*;
use uuid::Uuid;
use uuid::uuid;

use crate::db::establish_connection;
use crate::models::{self, Event, Join, Request, User, Solo, Team, NewEvent, NewJoin, NewRequest, NewUser, NewSolo, NewTeam};
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
    let binding = conv_string_to_uuid(team_id);
    let team_id = match &binding {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let binding = conv_string_to_uuid(user_id);
    let user_id = match &binding {
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
    let binding = conv_string_to_uuid(team_id);
    let team_id = match &binding {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let binding = conv_string_to_uuid(user_id);
    let user_id = match &binding {
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
    let binding = conv_string_to_uuid(event_id);
    let event_id = match &binding {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let binding = conv_string_to_uuid(user_id);
    let user_id = match &binding {
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
    let binding = conv_string_to_uuid(event_id);
    let event_id = match &binding {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let binding = conv_string_to_uuid(reader_id);
    let reader_id = match &binding {
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

pub fn get_event_list() -> anyhow::Result<Vec<Event>> {
    use crate::schema::events::dsl::events;
    let conn = &mut establish_connection().unwrap();
    match events.select(Event::as_select()).load(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e)),
    }
}
pub fn get_requests_from_user_id(user_id: &String) -> anyhow::Result<Vec<Request>> {
    let conn = &mut establish_connection()?;
    let user_id: Uuid = match Uuid::parse_str(user_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };

    match requests::dsl::requests.filter(requests::dsl::user_id.eq(user_id)).load::<Request>(conn) {
        Ok(r) => return Ok(r),
        Err(e) => return Err(anyhow!("{}", e)),
    };
} 

// TODO: impl these
pub fn get_wanna_join_users_by_event_id(event_id: &String) -> anyhow::Result<Vec<User>> {
    let conn = &mut establish_connection()?;
    let event_id: Uuid = match Uuid::parse_str(event_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let solo_vec: Vec<Solo> = match solos::dsl::solos
            .filter(solos::dsl::event_id.eq(event_id))
            .load::<Solo>(conn) {
        Ok(r) => r,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let mut result: Vec<User> = vec![];
    // Unpack vector
    for s in solo_vec {
        result.push(match search_user_by_id(s.user_id){
            Ok(u) => u,
            Err(e) => return Err(anyhow!("{}", e)),
        });
    }

    return Ok(result);
} 


pub fn get_user_info_by_name(user_name: &String) -> anyhow::Result<User> {
    let conn = &mut establish_connection()?;
    match search_user_by_name(user_name) {
        Ok(u) => return Ok(u),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn get_user_info_by_id(user_id: &String) -> anyhow::Result<User> {
    let conn = &mut establish_connection()?;
    let binding = conv_string_to_uuid(user_id);
    let user_id = match binding {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    match search_user_by_id(user_id) {
        Ok(u) => return Ok(u),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn get_team_info_by_id(team_id: &String) -> anyhow::Result<Team> {
    let conn = &mut establish_connection()?;
    let binding = conv_string_to_uuid(team_id);
    let team_id = match binding {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    match search_team_by_id(team_id) {
        Ok(u) => return Ok(u),
        Err(e) => Err(anyhow!("{}", e)),
    }
}

pub fn get_wanna_join_teams_by_event_id(event_id: &String) -> anyhow::Result<Vec<Team>> {
    let conn = &mut establish_connection()?;
    let event_id: Uuid = match Uuid::parse_str(event_id) {
        Ok(u) => u,
        Err(e) => return Err(anyhow!("{}", e)),
    };
    let team_vec: Vec<Team> = match teams::dsl::teams
            .filter(teams::dsl::event_id.eq(event_id))
            .load::<Team>(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e)),
    };
}
 
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

// Search
fn search_event_by_id(event_id: Uuid) -> anyhow::Result<Event> {
    let conn = &mut establish_connection()?;
    match events::dsl::events
        .filter(events::dsl::id.eq(event_id))
        .limit(1)
        .first::<Event>(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e)),
    }
}

fn search_team_by_id(team_id: Uuid) -> anyhow::Result<Team> {
    let conn = &mut establish_connection()?;
    match teams::dsl::teams
        .filter(teams::dsl::id.eq(team_id))
        .limit(1)
        .first::<Team>(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e)),
    }
}


fn search_user_by_name(name: &String) -> anyhow::Result<User> {
    let conn = &mut establish_connection()?;
    match users::dsl::users
        .filter(users::dsl::name.eq(name))
        .limit(1)
        .first::<User>(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e)),
    }
}

fn search_user_by_id(id: Uuid) -> anyhow::Result<User> {
    let conn = &mut establish_connection()?;
    match users::dsl::users
        .filter(users::dsl::id.eq(id))
        .limit(1)
        .first::<User>(conn) {
        Ok(v) => return Ok(v),
        Err(e) => return Err(anyhow!("{}", e)),
    }
}

// Utils
fn conv_string_to_uuid(str_uuid: &str) -> anyhow::Result<(Uuid)> {
    match Uuid::parse_str(str_uuid) {
        Ok(uuid) => return Ok(uuid),
        Err(e) => return Err(anyhow!("{}", e)),
    };
}
