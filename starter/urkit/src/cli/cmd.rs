use super::cfg::Cfg;
use super::opt::Opt;

use serde::Deserialize;

pub struct Cmd {
  pub conf: Cfg,
  pub kind: CmdKind,
}

impl Cmd {
  #[inline]
  pub fn new(conf: Cfg, opt: &Opt) -> Self {
    Self {
      conf,
      kind: CmdKind::from(opt),
    }
  }

  #[inline]
  pub fn kind(&self) -> &CmdKind {
    &self.kind
  }
}

#[derive(Debug, Deserialize)]
pub enum CmdKind {
  Unknown,
  React(Opt),
  Svelte(Opt),
  Vue(Opt),
}

impl From<&Opt> for CmdKind {
  fn from(rhs: &Opt) -> Self {
    let opt = Opt {
      generate: format!("{}", rhs.generate),
      template: format!("{}", rhs.template),
      name: format!("{}", rhs.name),
    };

    match &*rhs.generate {
      "react" => Self::React(opt),
      "svelte" => Self::Svelte(opt),
      "vue" => Self::Vue(opt),
      _ => Self::Unknown,
    }
  }
}

impl CmdKind {
  #[inline]
  pub fn mode(&self) -> &str {
    match self {
      Self::React(opt) | Self::Svelte(opt) | Self::Vue(opt) => &opt.generate,
      Self::Unknown => "unknown",
    }
  }

  #[inline]
  pub fn name(&self) -> &str {
    match self {
      Self::React(opt) | Self::Svelte(opt) | Self::Vue(opt) => &opt.name,
      Self::Unknown => "unknown",
    }
  }

  #[inline]
  pub fn template(&self) -> &str {
    match self {
      Self::React(opt) | Self::Svelte(opt) | Self::Vue(opt) => &opt.template,
      Self::Unknown => "unknown",
    }
  }
}

#[inline]
pub fn make_cmd(cfg: Cfg, opt: &Opt) -> Result<Box<Cmd>, String> {
  Ok(box Cmd::new(cfg, opt))
}
