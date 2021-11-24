use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LeverResponse {
  names: Vec<String>,
  emails: Vec<Email>,
  links: Vec<Link>,
  positions: Vec<Position>,
  schools: Vec<School>,
  summary: Summary,
  location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
struct Email {
  canonical: String,
  value: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Summary {
  executive_summary: String,
  skills: String,
  work_time: WorkTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct WorkTime {
  years: i32,
  months: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct School {
  degree: String,
  org: String,
  summary: String,
  start: Option<Date>,
  end: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Position {
  is_current: bool,
  title: String,
  org: String,
  summary: String,
  start: Option<Date>,
  end: Option<Date>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Date {
  month: i32,
  year: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
  name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Link {
  url: String,
  domain: String,
}

fn default_string(inp: String) -> String {
  if inp.len() > 0 { inp } else { "none".to_string() }
}

fn format_date(date: Date) -> String {
  format!("{}/{}", date.month, date.year)
}

fn format_date_range(start: Option<Date>, end: Option<Date>, current: bool) -> String {
  if start.is_none() && end.is_none() { "unknown date range".to_string() }
  else {
    let start_str = match start {
      Some(date) => format_date(date),
      None => "unknown".to_string(),
    };

    let end_str = match end {
      Some(date) => format_date(date),
      None => {
        if current {
          "current"
        } else {
          "unknown"
        }.to_string()
      },
    };

    format!("{} - {}", start_str, end_str)
  }
}

pub fn print_lever_response(resp: LeverResponse) {
  println!("Candidate: {} ({}y, {}m work experience)", &resp.names.join(", "), resp.summary.work_time.years, resp.summary.work_time.months);
  println!("Location: {}", default_string(resp.location.name));
  println!("Executive Summary: {}", default_string(resp.summary.executive_summary));
  println!("Skills: {}", default_string(resp.summary.skills));

  let links_count = resp.links.len();
  let emails_count = resp.emails.len();

  println!("Found {} links ({})", links_count, &resp.links.iter().map(|e| e.url.to_string()).collect::<Vec<_>>().join(", "));
  println!("Found {} emails ({})", emails_count, &resp.emails.iter().map(|e| e.value.to_string()).collect::<Vec<_>>().join(", "));

  println!("Found {} positions", resp.positions.len());
  for Position { title, org, start, end, is_current, .. } in resp.positions {
    println!("  - {} @ {} ({})", title, org, format_date_range(start, end, is_current));
  }

  println!("Found {} schools", resp.schools.len());
  for School { degree, org, .. } in resp.schools {
    println!("  - {} @ {}", degree, org);
  }
}
