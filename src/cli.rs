use std::path::{PathBuf, Path};
use clap::{Parser, ValueHint};

#[derive(Parser, Debug)]
#[clap(version, author)]
pub struct Opts {
  #[clap(short = 'f', long, parse(from_os_str), validator(pdf_check), multiple_occurrences(true), value_hint = ValueHint::FilePath, conflicts_with = "interactive")]
  pub file: Option<Vec<PathBuf>>,
  #[clap(short = 'i', long, conflicts_with = "file", required_unless_present("file"))]
  pub interactive: bool,
}

pub fn cli() -> Opts {
  let opt = Opts::parse();
  opt
}

fn pdf_check(val: &str) -> Result<(), String> {
  let path = Path::new(val);
  let extension = path.extension().expect("expected file with extension");

  if extension == "pdf" {
    if path.exists() {
      Ok(())
    } else {
      Err(format!("the file \"{}\" must exist.", val))
    }
  } else {
    Err("the file format must be pdf.".to_string())
  }
}
