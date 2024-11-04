use diesel::prelude::*;
use diesel::result::Error as DieselError;

use super::errors::DatabaseError;
use super::models::*;

pub struct Database {
    conn: SqliteConnection,
}

impl Database {
    pub fn new(database_url: &str) -> Result<Self, DatabaseError> {
        SqliteConnection::establish(database_url)
            .map(|conn| Database { conn })
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))
    }

    pub fn create_project(&mut self, new_project: NewProject) -> Result<i32, DatabaseError> {
        use crate::db::schema::projects::dsl::*;

        self.conn
            .transaction(|conn| {
                if let Ok(_) = projects
                    .filter(name.eq(&new_project.name))
                    .select(Project::as_select())
                    .first::<Project>(conn)
                {
                    return Err(DatabaseError::AlreadyExists(format!(
                        "Project with name '{}' already exists",
                        new_project.name
                    )));
                }

                diesel::insert_into(projects)
                    .values(&new_project)
                    .execute(conn)
                    .map_err(DatabaseError::from)?;

                projects
                    .select(id)
                    .order(id.desc())
                    .first(conn)
                    .map_err(DatabaseError::from)
            })
            .map_err(|e: DatabaseError| match e {
                DatabaseError::Unknown(_) => {
                    DatabaseError::TransactionError("Failed to create project".to_string())
                }
                e => e,
            })
    }

    pub fn get_project_by_id(&mut self, project_id: i32) -> Result<Project, DatabaseError> {
        use crate::db::schema::projects::dsl::*;

        projects
            .filter(id.eq(project_id))
            .select(Project::as_select())
            .first(&mut self.conn)
            .map_err(|e| match e {
                DieselError::NotFound => {
                    DatabaseError::NotFound(format!("Project with id {} not found", project_id))
                }
                e => DatabaseError::from(e),
            })
    }

    pub fn get_project_by_name(&mut self, project_name: String) -> Result<Project, DatabaseError> {
        use crate::db::schema::projects::dsl::*;

        projects
            .filter(name.eq(project_name.clone()))
            .select(Project::as_select())
            .first(&mut self.conn)
            .map_err(|e| match e {
                DieselError::NotFound => {
                    DatabaseError::NotFound(format!("Project '{}' not found", project_name))
                }
                e => DatabaseError::from(e),
            })
    }

    pub fn update_project(
        &mut self,
        project_name: String,
        updated_project_name: String,
    ) -> Result<i32, DatabaseError> {
        use crate::db::schema::projects::dsl::*;

        self.conn.transaction(|conn| {
            let project = projects
                .filter(name.eq(&project_name))
                .select(Project::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => {
                        DatabaseError::NotFound(format!("Project '{}' not found", project_name))
                    }
                    e => DatabaseError::from(e),
                })?;

            if let Ok(_) = projects
                .filter(name.eq(&updated_project_name))
                .select(Project::as_select())
                .first::<Project>(conn)
            {
                return Err(DatabaseError::AlreadyExists(format!(
                    "Project with name '{}' already exists",
                    updated_project_name
                )));
            }

            diesel::update(projects)
                .filter(id.eq(project.id))
                .set(name.eq(updated_project_name))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(project.id)
        })
    }

    pub fn delete_project(&mut self, project_name: String) -> Result<i32, DatabaseError> {
        use crate::db::schema::projects::dsl::{id as project_id, name, projects};
        use crate::db::schema::subtasks::dsl::{subtasks, task_id as subtask_task_id};
        use crate::db::schema::tasks::dsl::{id as task_id, project_id as task_project_id, tasks};

        self.conn.transaction(|conn| {
            let project = projects
                .filter(name.eq(project_name.clone()))
                .select(Project::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => {
                        DatabaseError::NotFound(format!("Project '{}' not found", project_name))
                    }
                    e => DatabaseError::from(e),
                })?;

            let project_tasks = tasks
                .filter(task_project_id.eq(project.id))
                .select(task_id)
                .load::<i32>(conn)
                .map_err(DatabaseError::from)?;

            diesel::delete(subtasks)
                .filter(subtask_task_id.eq_any(project_tasks))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            diesel::delete(tasks)
                .filter(task_project_id.eq(project.id))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            diesel::delete(projects)
                .filter(project_id.eq(project.id))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(project.id)
        })
    }

    pub fn add_new_task(&mut self, new_task: NewTask) -> Result<i32, DatabaseError> {
        use crate::db::schema::tasks::dsl::*;
        self.get_project_by_id(new_task.project_id)?;

        self.conn.transaction(|conn| {
            if let Ok(_) = tasks
                .filter(name.eq(&new_task.name))
                .filter(project_id.eq(new_task.project_id))
                .select(Task::as_select())
                .first::<Task>(conn)
            {
                return Err(DatabaseError::AlreadyExists(format!(
                    "Task with name '{}' already exists",
                    new_task.name
                )));
            }

            diesel::insert_into(tasks)
                .values(new_task)
                .execute(conn)
                .map_err(DatabaseError::from)?;

            tasks
                .select(id)
                .order(id.desc())
                .first(conn)
                .map_err(DatabaseError::from)
        })
    }

    pub fn update_task(
        &mut self,
        req_task_name: String,
        update_task_name: String,
    ) -> Result<i32, DatabaseError> {
        use crate::db::schema::tasks::dsl::{id as task_id, name as task_name, tasks};

        self.conn.transaction(|conn| {
            let task = tasks
                .filter(task_name.eq(&req_task_name))
                .select(Task::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => {
                        DatabaseError::NotFound(format!("Task '{}' not found", req_task_name))
                    }
                    e => DatabaseError::from(e),
                })?;

            if let Ok(_) = tasks
                .filter(task_name.eq(&update_task_name))
                .select(Task::as_select())
                .first::<Task>(conn)
            {
                return Err(DatabaseError::AlreadyExists(format!(
                    "Task with name '{}' already exists",
                    update_task_name
                )));
            }

            diesel::update(tasks)
                .filter(task_id.eq(task.id))
                .set(task_name.eq(update_task_name))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(task.id)
        })
    }

    pub fn update_task_status(
        &mut self,
        req_task_name: String,
        completed: bool,
    ) -> Result<i32, DatabaseError> {
        use crate::db::schema::tasks::dsl::{
            completed as task_completed, name as task_name, tasks,
        };

        self.conn.transaction(|conn| {
            let task = tasks
                .filter(task_name.eq(&req_task_name))
                .select(Task::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => {
                        DatabaseError::NotFound(format!("Task '{}' not found", req_task_name))
                    }
                    e => DatabaseError::from(e),
                })?;

            diesel::update(tasks)
                .filter(task_name.eq(task.name))
                .set(task_completed.eq(completed))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(task.id)
        })
    }

    pub fn delete_task(&mut self, req_task_name: String) -> Result<i32, DatabaseError> {
        use crate::db::schema::subtasks::dsl::{subtasks, task_id as subtask_task_id};
        use crate::db::schema::tasks::dsl::{id as task_id, name as task_name, tasks};

        self.conn.transaction(|conn| {
            let task = tasks
                .filter(task_name.eq(&req_task_name))
                .select(Task::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => {
                        DatabaseError::NotFound(format!("Task '{}' not found", req_task_name))
                    }
                    e => DatabaseError::from(e),
                })?;

            diesel::delete(subtasks)
                .filter(subtask_task_id.eq(task.id))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            diesel::delete(tasks)
                .filter(task_id.eq(task.id))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(task.id)
        })
    }

    pub fn get_tasks_by_name(&mut self, req_task_name: String) -> Result<Task, DatabaseError> {
        use crate::db::schema::tasks::dsl::{name as task_name, tasks};

        tasks
            .filter(task_name.eq(&req_task_name))
            .select(Task::as_select())
            .first(&mut self.conn)
            .map_err(|e| match e {
                DieselError::NotFound => {
                    DatabaseError::NotFound(format!("Task '{}' not found", req_task_name))
                }
                e => DatabaseError::from(e),
            })
    }

    pub fn get_tasks_by_id(&mut self, req_task_id: i32) -> Result<Task, DatabaseError> {
        use crate::db::schema::tasks::dsl::{id as task_id, tasks};

        tasks
            .filter(task_id.eq(&req_task_id))
            .select(Task::as_select())
            .first(&mut self.conn)
            .map_err(|e| match e {
                DieselError::NotFound => {
                    DatabaseError::NotFound(format!("Task with id {} not found", req_task_id))
                }
                e => DatabaseError::from(e),
            })
    }

    pub fn add_new_subtask(&mut self, new_subtask: NewSubTask) -> Result<i32, DatabaseError> {
        use crate::db::schema::subtasks::dsl::*;

        self.conn.transaction(|conn| {
            // First check if subtask with this name exists under the task
            if let Ok(_) = subtasks
                .filter(name.eq(&new_subtask.name))
                .filter(task_id.eq(new_subtask.task_id))
                .select(SubTask::as_select())
                .first::<SubTask>(conn)
            {
                return Err(DatabaseError::AlreadyExists(format!(
                    "Subtask with name '{}' already exists for this task",
                    new_subtask.name
                )));
            }

            // If not, create the new subtask
            diesel::insert_into(subtasks)
                .values(new_subtask)
                .execute(conn)
                .map_err(DatabaseError::from)?;

            subtasks
                .select(id)
                .order(id.desc())
                .first(conn)
                .map_err(DatabaseError::from)
        })
    }

    pub fn update_subtask(
        &mut self,
        req_task_id: i32,
        req_subtask_id: i32,
        req_subtask_name: String,
    ) -> Result<i32, DatabaseError> {
        use crate::db::schema::subtasks::dsl::{
            id as subtask_id, name as subtask_name, subtasks, task_id as subtask_task_id,
        };

        self.conn.transaction(|conn| {
            let subtask = subtasks
                .filter(subtask_id.eq(&req_subtask_id))
                .filter(subtask_task_id.eq(&req_task_id))
                .select(SubTask::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => DatabaseError::NotFound(format!(
                        "Subtask {} for task {} not found",
                        req_subtask_id, req_task_id
                    )),
                    e => DatabaseError::from(e),
                })?;

            if let Ok(_) = subtasks
                .filter(subtask_name.eq(&req_subtask_name))
                .filter(subtask_task_id.eq(&req_task_id))
                .select(SubTask::as_select())
                .first::<SubTask>(conn)
            {
                return Err(DatabaseError::AlreadyExists(format!(
                    "Subtask with name '{}' already exists for this task",
                    req_subtask_name
                )));
            }

            diesel::update(subtasks)
                .filter(subtask_task_id.eq(&req_task_id))
                .filter(subtask_id.eq(&req_subtask_id))
                .set(subtask_name.eq(req_subtask_name))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(subtask.id)
        })
    }

    pub fn get_subtask_by_id(&mut self, req_subtask_id: i32) -> Result<SubTask, DatabaseError> {
        use crate::db::schema::subtasks::dsl::{id as subtask_id, subtasks};

        subtasks
            .filter(subtask_id.eq(&req_subtask_id))
            .select(SubTask::as_select())
            .first(&mut self.conn)
            .map_err(|e| match e {
                DieselError::NotFound => {
                    DatabaseError::NotFound(format!("Subtask {} not found", req_subtask_id))
                }
                e => DatabaseError::from(e),
            })
    }

    pub fn delete_subtask(
        &mut self,
        req_subtask_id: i32,
        req_task_id: i32,
    ) -> Result<i32, DatabaseError> {
        use crate::db::schema::subtasks::dsl::{
            id as subtask_id, subtasks, task_id as task_subtask_id,
        };

        self.conn.transaction(|conn| {
            let subtask = subtasks
                .filter(task_subtask_id.eq(&req_task_id))
                .filter(subtask_id.eq(&req_subtask_id))
                .select(SubTask::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => DatabaseError::NotFound(format!(
                        "Subtask {} for task {} not found",
                        req_subtask_id, req_task_id
                    )),
                    e => DatabaseError::from(e),
                })?;

            diesel::delete(subtasks)
                .filter(task_subtask_id.eq(&req_task_id))
                .filter(subtask_id.eq(&req_subtask_id))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(subtask.id)
        })
    }

    pub fn update_subtask_status(
        &mut self,
        completed: bool,
        req_subtask_id: i32,
        req_task_id: i32,
    ) -> Result<i32, DatabaseError> {
        use crate::db::schema::subtasks::dsl::{
            completed as subtask_completed, id as subtask_id, subtasks, task_id as subtask_task_id,
        };

        self.conn.transaction(|conn| {
            let subtask_result = subtasks
                .filter(subtask_task_id.eq(&req_task_id))
                .filter(subtask_id.eq(&req_subtask_id))
                .select(SubTask::as_select())
                .first(conn)
                .map_err(|e| match e {
                    DieselError::NotFound => DatabaseError::NotFound(format!(
                        "Subtask {} for task {} not found",
                        req_subtask_id, req_task_id
                    )),
                    e => DatabaseError::from(e),
                })?;

            diesel::update(subtasks)
                .filter(subtask_task_id.eq(&req_task_id))
                .filter(subtask_id.eq(&req_subtask_id))
                .set(subtask_completed.eq(&completed))
                .execute(conn)
                .map_err(DatabaseError::from)?;

            Ok(subtask_result.id)
        })
    }

    pub fn get_task_by_name_and_project_id(
        &mut self,
        task_name: String,
        project_id: i32,
    ) -> Result<Task, DatabaseError> {
        use crate::db::schema::tasks::dsl::{name, project_id as task_project_id, tasks};

        tasks
            .into_boxed()
            .filter(name.eq(task_name.clone()))
            .filter(task_project_id.eq(project_id))
            .select(Task::as_select())
            .first(&mut self.conn)
            .map_err(|e| match e {
                DieselError::NotFound => {
                    DatabaseError::NotFound(format!("Task {} not found in this project", task_name))
                }
                _ => e.into(),
            })
    }

    pub fn get_all_tasks(&mut self, req_project_id: i32) -> Result<Vec<Task>, DatabaseError> {
        use crate::db::schema::tasks::dsl::{project_id as task_project_id, tasks};

        tasks
            .into_boxed()
            .filter(task_project_id.eq(req_project_id))
            .select(Task::as_select())
            .load(&mut self.conn)
            .map_err(DatabaseError::from)
    }
}
