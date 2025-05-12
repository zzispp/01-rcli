use super::verify_dir;
use clap::Parser;
#[derive(Debug, Parser)]
pub enum HttpSubCommand {
    #[command(about = "Serve a directory over HTTP")]
    Serve(HttpServeOpts),
}

#[derive(Debug, Parser)]
pub struct HttpServeOpts {
    #[arg(short,long,value_parser = verify_dir,default_value = ".")]
    pub dir: String,
    #[arg(short, long, default_value_t = 8080)]
    pub port: u16,
}
