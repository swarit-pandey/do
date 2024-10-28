CREATE TABLE projects (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL UNIQUE,
    created_on TIMESTAMP NOT NULL
);

CREATE TABLE tasks (
    id INTEGER PRIMARY KEY NOT NULL,
    project_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_on TIMESTAMP NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE subtasks (
    id INTEGER PRIMARY KEY NOT NULL,
    task_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT FALSE,
    created_on TIMESTAMP NOT NULL,
    FOREIGN KEY (task_id) REFERENCES tasks(id)
);

CREATE TABLE thoughts (
    id INTEGER PRIMARY KEY NOT NULL,
    heading TEXT NOT NULL,
    note TEXT NOT NULL,
    created_on TIMESTAMP NOT NULL
);
