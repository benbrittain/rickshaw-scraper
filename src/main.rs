use anyhow::Error;
use std::io::Write;
use rayon::prelude::*;
use regex::Regex;

const START: u32 = 624687;
const RANGE: u32 = 18000;

fn main() -> Result<(), Error> {
    let client = reqwest::Client::new();
    let re = Regex::new(r###"pubdate" datetime="(.*)">"###).unwrap();
    (START..START+RANGE).into_par_iter().try_for_each(|i| {
        let url = format!("https://blogging.theadventurists.com/tracking/updates/show/{}/", i);
        let body = client.get(&url).query(&[("_bare/", "1")]).send()?.text()?;
        if body.contains("Rickshaw Run") {
            for cap in re.captures_iter(&body) {
                let date = &cap[1];
                eprintln!("checked {}. Date: {}", i, chrono::DateTime::parse_from_rfc3339(date)?);
            }
        }
        if body.contains("Benjamin Brittain") {
            let fout = format!("out_{}.html", i);
            let mut file = std::fs::File::create(fout)?;
            file.write_all(body.as_bytes())?;
            eprintln!("Found & Saved.");
        }
        Ok::<(), Error>(())
    })
}
