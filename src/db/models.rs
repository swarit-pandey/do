use crate::db::schema::*;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, QueryableByName, Identifiable, Selectable)]
#[diesel(table_name = projects)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub created_on: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = projects)]
pub struct NewProject {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(Project))]
#[diesel(table_name = tasks)]
pub struct Task {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub completed: bool,
    pub created_on: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = tasks)]
pub struct NewTask {
    pub project_id: i32,
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(belongs_to(Task))]
#[diesel(table_name = subtasks)]
pub struct SubTask {
    pub id: i32,
    pub task_id: i32,
    pub name: String,
    pub completed: bool,
    pub created_on: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = subtasks)]
pub struct NewSubTask {
    pub task_id: i32,
    pub name: String,
    pub completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[diesel(table_name = thoughts)]
pub struct Thoughts {
    pub id: i32,
    pub heading: String,
    pub note: String,
    pub created_on: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = thoughts)]
pub struct NewThought {
    pub heading: String,
    pub note: String,
}
