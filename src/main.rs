use std::{fs, process::exit};
use dialoguer::{
  MultiSelect,
  theme::ColorfulTheme
};

mod print;
use print::{LeverResponse, print_lever_response};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    .items(&files.iter().map(|f| f.file_name().unwrap().to_str().unwrap()).collect::<Vec<_>>())
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
    let input_file_name = files[selection].file_name().unwrap().to_str().unwrap();

    let file_contents = fs::read(&files[selection]).unwrap();
    let part = reqwest::multipart::Part::bytes(file_contents)
      .file_name(input_file_name.to_string());

    let form = reqwest::multipart::Form::new()
      .part("resume", part);

    let client = reqwest::Client::new();
    let resp = client.post("https://jobs.lever.co/parseResume")
      .header("referer", "https://jobs.lever.co/")
      .header("origin", "https://jobs.lever.co/")
      .header("cookie", "lever-referer=https://jobs.lever.co/")
      .multipart(form)
      .send()
      .await?;

    let content = resp.json::<serde_json::Value>().await?;
    
    println!("-----");
    let lever_response: LeverResponse = serde_json::from_value(content.clone()).expect("could not decode lever response");
    print_lever_response(lever_response);
    println!("-----");

    let output = serde_json::to_string_pretty(&content)?;

    let file_stem = files[selection].file_stem().unwrap().to_str().unwrap();
    let output_file_name= format!("{}.json", file_stem);
    fs::write(&output_file_name, output).expect("could not write file");

    println!("Wrote output of `{}` to `{}`", &input_file_name, &output_file_name);
  }

  Ok(())
}
