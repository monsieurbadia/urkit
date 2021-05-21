use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Cfg {
  single: Option<SingleCfg>,
  multiple: Option<MultipleCfg>,
  wasm: Option<WasmCfg>,
}

impl Cfg {
  pub fn single(&self) -> &SingleCfg {
    &self.single.as_ref().unwrap()
  }

  pub fn multiple(&self) -> &MultipleCfg {
    &self.multiple.as_ref().unwrap()
  }

  pub fn wasm(&self) -> &WasmCfg {
    &self.wasm.as_ref().unwrap()
  }
}

#[derive(Debug, Deserialize)]
pub enum ConfKind {
  Unknown,
  Single(SingleCfg),
  Multiple(MultipleCfg),
  Wasm(WasmCfg),
}

#[derive(Default, Debug, Deserialize)]
pub struct SingleCfg {
  pub component: Option<String>,
  pub spec: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct MultipleCfg {
  pub component: Option<String>,
  pub html: Option<String>,
  pub style: Option<String>,
  pub script: Option<String>,
  pub spec: Option<String>,
}

#[derive(Default, Debug, Deserialize)]
pub struct WasmCfg {}
