use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'w', long = "word", group = "mode", default_value_t = String::new())]
    pub word: String,

    #[arg(short = 'r', long = "random", group = "mode", default_value_t = false)]
    pub random: bool,

    #[arg(short = 'D', long = "difficult", default_value_t = false)]
    pub difficult: bool,

    #[arg(short = 't', long = "stats", default_value_t = false)]
    pub stats: bool,

    #[arg(short = 'd', long = "day", default_value_t = 1)]
    pub day: usize,

    #[arg(short = 's', long = "seed", default_value_t = 19260817)]
    pub seed: u64,

    #[arg(short = 'f', long = "final-set", default_value_t = String::from("./final_set.txt"))]
    pub final_set: String,

    #[arg(short = 'a', long = "acceptable-set", default_value_t = String::from("./acceptable_set.txt"))]
    pub acceptable_set: String,

    #[arg(short = 'S', long = "state", default_value_t = String::from("./state.json"))]
    pub state: String,

    #[arg(short = 'c', long = "config", default_value_t = String::from("./config.json"))]
    pub config: String,
}