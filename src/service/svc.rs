use crate::db::{errors::DatabaseError, models::*, operations::Database};

use super::errors::ServiceError;

pub struct DoitService {
    db: Database,
}

impl DoitService {
    pub fn new(database_url: &str) -> Result<Self, ServiceError> {
        let db = Database::new(database_url)
            .map_err(|e| ServiceError::Database(DatabaseError::ConnectionError(e.to_string())))?;
        Ok(DoitService { db })
    }

    pub fn add_new_project(&mut self, project_name: String) -> Result<i32, ServiceError> {
        if project_name.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Project name cannot be empty".to_string(),
            ));
        }

        let new_project = NewProject { name: project_name };

        Ok(self.db.create_project(new_project)?)
    }

    pub fn add_task(
        &mut self,
        project_name: String,
        task_name: String,
    ) -> Result<i32, ServiceError> {
        if task_name.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Task name cannot be empty".to_string(),
            ));
        }

        let project = self.db.get_project_by_name(project_name)?;

        let new_task = NewTask {
            project_id: project.id,
            name: task_name,
            completed: false,
        };

        Ok(self.db.add_new_task(new_task)?)
    }

    pub fn add_subtask(
        &mut self,
        project_name: String,
        task_name: String,
        subtask_name: String,
    ) -> Result<i32, ServiceError> {
        if subtask_name.trim().is_empty() {
            return Err(ServiceError::InvalidInput(
                "Subtask name cannot be empty".to_string(),
            ));
        }

        let project = self
            .db
            .get_project_by_name(project_name.clone())
            .map_err(|e| match e {
                DatabaseError::NotFound(_) => {
                    ServiceError::InvalidInput(format!("Project '{}' does not exist", project_name))
                }
                e => e.into(),
            })?;

        let task = self
            .db
            .get_task_by_name_and_project_id(task_name.clone(), project.id)
            .map_err(|e| match e {
                DatabaseError::NotFound(_) => ServiceError::InvalidInput(format!(
                    "Task '{}' does not exist in project '{}'",
                    task_name, project_name
                )),
                e => e.into(),
            })?;

        let new_subtask = NewSubTask {
            task_id: task.id,
            name: subtask_name,
            completed: false,
        };

        Ok(self.db.add_new_subtask(new_subtask)?)
    }

    pub fn get_all_tasks(&mut self, project_name: String) -> Result<Vec<Task>, ServiceError> {
        let project = self
            .db
            .get_project_by_name(project_name.clone())
            .map_err(|e| match e {
                DatabaseError::NotFound(_) => {
                    ServiceError::InvalidInput(format!("Project '{}' does not exist", project_name))
                }
                e => e.into(),
            })?;

        Ok(self.db.get_all_tasks(project.id)?)
    }
}
