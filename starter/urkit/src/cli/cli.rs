use std::env;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;

use rayon::prelude::ParallelIterator;
use rayon::prelude::*;
use structopt::StructOpt;
use uwa::fp::and_then;
use uwa::pattern::unwrap_or;

use super::cfg::{Cfg, MultipleCfg, SingleCfg, WasmCfg};
use super::cmd::{make_cmd, Cmd, CmdKind};
use super::opt::Opt;
use super::src::Src;

const STACK_SIZE: usize = 128 * 1024 * 1024;

#[inline]
pub fn run() {
  let opt = Opt::from_args();

  let ok = thread::Builder::new()
    .stack_size(STACK_SIZE)
    .spawn(|| bundle(opt))
    .unwrap()
    .join()
    .unwrap();

  std::process::exit(match ok {
    Ok(()) => 0,
    Err(..) => 2,
  });
}

#[inline]
fn bundle(opt: Opt) -> Result<(), String> {
  let cmd = parse_cmd(&opt)?;
  let out = parse_outdir(&cmd)?;
  let tmp = parse_template(&cmd)?;

  and_then!(create_files, out, tmp)
}

#[inline]
fn parse_cmd(opt: &Opt) -> Result<Box<Cmd>, String> {
  let cfg = read_conf_toml(&opt.generate, &opt.name)?;

  and_then!(make_cmd, cfg, opt)
}

#[inline]
fn parse_template(cmd: &Cmd) -> Result<Vec<Src>, String> {
  match cmd.kind() {
    CmdKind::React(_) => parse_component(cmd),
    CmdKind::Svelte(_) => parse_component(cmd),
    CmdKind::Vue(_) => parse_component(cmd),
    _ => Err(format!("generate command not found")),
  }
}

#[inline]
fn parse_component(cmd: &Cmd) -> Result<Vec<Src>, String> {
  match cmd.kind.template() {
    "single" => parse_single_conf(cmd, cmd.conf.single()),
    "multiple" => parse_multiple_conf(cmd, cmd.conf.multiple()),
    "wasm" => parse_wasm_conf(cmd, cmd.conf.wasm()),
    _ => Err(format!("template command not found")),
  }
}

#[inline]
fn parse_single_conf(
  cmd: &Cmd,
  current: &SingleCfg,
) -> Result<Vec<Src>, String> {
  let mut sources = vec![];
  let mode = cmd.kind.mode();
  let name = cmd.kind.name();
  let dest = parse_outdir(cmd)?;

  let component = unwrap_or!(return: &current.component, Ok(vec![]));
  if !component.is_empty() {
    let ext = if mode != "react" { mode } else { "jsx" };
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", component),
      dest: format!("{}/{}.{}", dest, name, ext),
    })
  }

  let spec = unwrap_or!(return: &current.spec, Ok(vec![]));
  if !spec.is_empty() {
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", spec),
      dest: format!("{}/{}.{}.js", dest, name, "spec"),
    })
  }

  Ok(sources)
}

#[inline]
fn parse_multiple_conf(
  cmd: &Cmd,
  current: &MultipleCfg,
) -> Result<Vec<Src>, String> {
  let mut sources = vec![];
  let mode = cmd.kind.mode();
  let name = cmd.kind.name();
  let dest = parse_outdir(cmd)?;

  let component = unwrap_or!(return: &current.component, Ok(vec![]));
  if !component.is_empty() {
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", component),
      dest: format!("{}/{}.{}", dest, name, mode),
    })
  }

  let html = unwrap_or!(return: &current.html, Ok(vec![]));
  if !html.is_empty() {
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", html),
      dest: format!("{}/{}.{}", dest, name, "html"),
    })
  }

  let style = unwrap_or!(return: &current.style, Ok(vec![]));
  if !style.is_empty() {
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", style),
      dest: format!("{}/{}.{}", dest, name, "css"),
    })
  }

  let script = unwrap_or!(return: &current.script, Ok(vec![]));
  if !script.is_empty() {
    let ext = if mode != "react" { "js" } else { "jsx" };
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", script),
      dest: format!("{}/{}.{}", dest, name, ext),
    })
  }

  let spec = unwrap_or!(return: &current.spec, Ok(vec![]));
  if !spec.is_empty() {
    sources.push(Src {
      name: format!("{}", name),
      text: format!("{}", spec),
      dest: format!("{}/{}.{}.js", dest, name, "spec"),
    })
  }

  Ok(sources)
}

#[inline]
fn parse_wasm_conf(
  _cmd: &Cmd,
  _current: &WasmCfg,
) -> Result<Vec<Src>, String> {
  Ok(vec![])
}

#[inline]
fn parse_outdir(cmd: &Cmd) -> Result<String, String> {
  let cdir = format!("{}", env::current_dir().unwrap().display());
  let dest = format!("{}/{}", cdir, cmd.kind.name());

  Ok(dest)
}

#[inline]
fn create_file(src: &Src) {
  let mut buf = File::create(&src.dest).unwrap();
  let bufbytes = src.text.as_bytes();

  buf.write_all(bufbytes).expect("cannot write to file");
}

#[inline]
fn create_files(dest: String, sources: Vec<Src>) -> Result<(), String> {
  let (tx, rx) = flume::unbounded();
  let d = dest.to_string();

  thread::spawn(move || {
    fs::create_dir(dest).unwrap_or(());
    sources.par_iter().for_each(|s| {
      tx.send(create_file(s)).unwrap();
    });
  })
  .join()
  .unwrap();

  match rx.recv() {
    Ok(()) => Ok(print!("\ncreated at: {}\n", d)),
    Err(e) => Err(format!("cannot create files: {}", e)),
  }
}

#[inline]
fn read_conf_toml(mode: &str, name: &str) -> Result<Cfg, String> {
  let pathname = format!("src/toml/{}.conf.toml", mode);
  let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(pathname);

  and_then!(read_conf, &path, name)
}

#[inline]
fn read_conf(path: &Path, query: &str) -> Result<Cfg, String> {
  match fs::read_to_string(&path) {
    Ok(ref f) => Ok(toml::from_str(&f.replace("{0}", query)).unwrap()),
    Err(e) => Err(format!("read conf error {}", e)),
  }
}

