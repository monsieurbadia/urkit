use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Src {
  pub name: String,
  pub text: String,
  pub dest: String,
}
