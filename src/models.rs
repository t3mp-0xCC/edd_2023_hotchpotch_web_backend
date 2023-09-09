use chrono::NaiveDateTime;
use diesel::prelude::{Queryable, Selectable, Insertable};
use diesel_derives::Identifiable;
use serde::Serialize;
use uuid::Uuid;

use crate::schema::{
    events,
    joins,
    requests,
    teams,
    users,
    solos,
};

#[derive(Queryable, Serialize, Identifiable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = events)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub desc: String,
    pub url: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = events)]
pub struct  NewEvent<'a> {
    pub name: &'a String,
    pub desc: &'a String,
    pub url: &'a String,
}

#[derive(Queryable, Serialize, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = joins)]
pub struct Join {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = joins)]
pub struct  NewJoin<'a> {
    pub team_id: &'a Uuid,
    pub user_id: &'a Uuid,
}

#[derive(Queryable, Serialize, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = requests)]
pub struct Request {
    pub team_id: Uuid,
    pub user_id: Uuid,
    pub message: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = requests)]
pub struct  NewRequest<'a> {
    pub team_id: &'a Uuid,
    pub user_id: &'a Uuid,
    pub message: &'a String,
}

#[derive(Queryable, Serialize, Identifiable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub icon_url: String,
    pub profile: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct  NewUser<'a> {
    pub name: &'a String,
    pub icon_url: &'a String,
    pub profile: &'a String,
}

#[derive(Queryable, Serialize, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = solos)]
pub struct Solo {
    pub event_id: Uuid,
    pub user_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = solos)]
pub struct  NewSolo<'a> {
    pub event_id: &'a Uuid,
    pub user_id: &'a Uuid,
}

#[derive(Queryable, Serialize, Identifiable, Selectable, PartialEq, Debug, Clone)]
#[diesel(table_name = teams)]
pub struct Team {
    pub id: Uuid,
    pub event_id: Uuid,
    pub reader_id: Uuid,
    pub name: String,
    pub desc: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = teams)]
pub struct  NewTeam<'a> {
    pub event_id: &'a Uuid,
    pub reader_id: &'a Uuid,
    pub name: &'a String,
    pub desc: &'a String,
}


