-- up.sql
CREATE UNIQUE INDEX idx_unique_task_name_per_project 
ON tasks(project_id, name);

CREATE UNIQUE INDEX idx_unique_subtask_name_per_task 
ON subtasks(task_id, name);
