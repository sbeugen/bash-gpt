use clap::Parser;

/// Program to get Bash command by providing a prompt in natural language
/// It expects the environment variable OPEN_AI_API_KEY to be set
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, trailing_var_arg=true)]
pub struct Arguments {
    /// If set the result will include explanations for the command
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,

    /// Query for command in natural language
    #[arg()]
    pub query: Vec<String>,
}

pub fn get() -> Arguments {
    Arguments::parse()
}
