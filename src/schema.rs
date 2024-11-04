// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Integer,
        name -> Text,
        created_on -> Timestamp,
    }
}

diesel::table! {
    subtasks (id) {
        id -> Integer,
        task_id -> Integer,
        name -> Text,
        completed -> Bool,
        created_on -> Timestamp,
    }
}

diesel::table! {
    tasks (id) {
        id -> Integer,
        project_id -> Integer,
        name -> Text,
        completed -> Bool,
        created_on -> Timestamp,
    }
}

diesel::table! {
    thoughts (id) {
        id -> Integer,
        heading -> Text,
        note -> Text,
        created_on -> Timestamp,
    }
}

diesel::joinable!(subtasks -> tasks (task_id));
diesel::joinable!(tasks -> projects (project_id));

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    subtasks,
    tasks,
    thoughts,
);
