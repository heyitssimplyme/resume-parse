use std::{fs, process::exit};
use dialoguer::{
  MultiSelect,
  theme::ColorfulTheme
};

mod print;
mod cli;
mod parse;

use cli::cli;
use parse::parse_resume;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  let opts = cli();

  if opts.interactive {
    let files = fs::read_dir(".").unwrap()
      .filter_map(Result::ok)
      .filter_map(|res|
        match res.path().extension() {
          Some(ext) => {
            (ext == "pdf").then(|| res.path())
          },
          None => None
        }
      )
      .collect::<Vec<_>>();

    if files.len() == 0 {
      println!("No pdf files found!");
      exit(0);
    }

    println!("Select your pdf files");

    let chosen = MultiSelect::with_theme(&ColorfulTheme::default())
      .items(&files.iter().map(|f| f.file_name().unwrap().to_str().unwrap()).collect::<Vec<&str>>())
      .defaults(&[true]) // keep the first one selected
      .interact()?;

    if chosen.len() == 0 {
      println!("No files selected!");
      exit(0);
    } else {
      let chosen_file_names = chosen
        .iter()
        .map(|&i| files[i].file_name().unwrap().to_str().unwrap())
        .collect::<Vec<_>>()
        .join(", ");
      println!("Chosen files: {}", chosen_file_names);
    }

    for selection in chosen {
      parse_resume(&files[selection]).await?;
    }
  } else {
    for file in opts.file.unwrap_or(Vec::new()) {
      parse_resume(&file).await?;
    }
  }

  Ok(())
}
