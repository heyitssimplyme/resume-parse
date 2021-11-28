use std::{fs, path::PathBuf};

use crate::print::{LeverResponse, print_lever_response};

pub async fn parse_resume (path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
  let file_name = path.file_name().unwrap().to_str().unwrap();
  let file_stem = path.file_stem().unwrap().to_str().unwrap();

  let file_contents = fs::read(&path).unwrap();
  let part = reqwest::multipart::Part::bytes(file_contents)
    .file_name(file_name.to_string());

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

  let output_file_name= format!("{}.json", file_stem);
  fs::write(&output_file_name, output).expect("could not write file");

  println!("Wrote output of `{}` to `{}`", file_name, output_file_name);

  Ok(())
}
