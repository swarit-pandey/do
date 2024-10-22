use clap::Subcommand;

// `doit add task --project <some-name> --task <some-task>`
// `doit add project --project <some-name>`
// `doit add point --project <some-name> --point <some-point>`
// `doit add thought <some-thought>`

#[derive(Subcommand)]
pub enum AddCommands {
    Task {
        #[arg(long)]
        project: String,
        #[arg(long)]
        task: String,
    },

    Project {
        #[arg(long)]
        project: String,
    },

    Point {
        #[arg(long)]
        point: String,
    },

    Thought {
        #[arg(long)]
        thought: String,
    },
}
