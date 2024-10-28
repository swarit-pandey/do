use diesel::prelude::*;
use diesel::result::Error;

use super::models::*;

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    pub fn new(database_url: &str) -> Result<Self, ConnectionError> {
        let conn = SqliteConnection::establish(database_url)?;
        return Ok(Database { conn });
    }

    // Project operations
    pub fn create_project(&mut self, new_project: NewProject) -> Result<i32, Error> {
        use crate::db::schema::projects::dsl::*;

        let _ = diesel::insert_into(projects)
            .values(&new_project)
            .execute(&mut self.conn);

        projects.select(id).order(id.desc()).first(&mut self.conn)
    }

    pub fn get_project_by_id(&mut self, project_id: i32) -> Result<Project, Error> {
        use crate::db::schema::projects::dsl::*;

        projects
            .filter(id.eq(project_id))
            .select(Project::as_select())
            .first(&mut self.conn)
    }

    pub fn get_project_by_name(&mut self, project_name: String) -> Result<Project, Error> {
        use crate::db::schema::projects::dsl::*;

        projects
            .filter(name.eq(project_name))
            .select(Project::as_select())
            .first(&mut self.conn)
    }
}
