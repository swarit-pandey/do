use crate::service::errors::ServiceError;
use crate::service::svc::DoitService;

use super::{
    add::AddCommands,
    types::{Cli, Commands},
};

pub struct CommandHandler {
    service: DoitService,
}

impl CommandHandler {
    pub fn new(database_url: &str) -> Result<Self, ServiceError> {
        let service = DoitService::new(database_url)?;
        Ok(CommandHandler { service })
    }

    pub fn handle_command(&mut self, cli: Cli) -> Result<String, ServiceError> {
        match cli.command {
            Commands::Add { subcmd } => self.handle_add_command(subcmd),
            Commands::Update {} => Ok("Update command to be implemented".to_string()),
        }
    }

    fn handle_add_command(&mut self, command: AddCommands) -> Result<String, ServiceError> {
        match command {
            AddCommands::Project { project } => {
                self.service.add_new_project(project.clone())?;
                Ok(format!("Successfully created project '{}'", project))
            }
            AddCommands::Task { project, task } => {
                self.service.add_task(project.clone(), task.clone())?;
                Ok(format!(
                    "Successfully added task '{}' to project '{}'",
                    task, project
                ))
            }
            AddCommands::Point { point: _ } => Ok("Point command not implemented yet".to_string()),
            AddCommands::Thought { thought: _ } => {
                Ok("Thought command not implemented yet".to_string())
            }
        }
    }
}
