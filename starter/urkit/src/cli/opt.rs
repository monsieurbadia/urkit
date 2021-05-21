use serde::Deserialize;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Deserialize)]
#[structopt(name = "urkit")]
pub struct Opt {
  #[structopt(short, long)]
  pub generate: String,
  #[structopt(short, long)]
  pub template: String,
  #[structopt(short, long)]
  pub name: String,
}
