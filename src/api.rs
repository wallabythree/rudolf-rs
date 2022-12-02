use reqwest::header::COOKIE;
use reqwest::header::USER_AGENT;

pub fn get(
    session: &str,
    year: u16,
    day: u8
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);
    let res = client
        .get(url)
        .header(
            USER_AGENT,
            "rudolf-rs 0.1 (https://github.com/wallabythree/rudolf-rs)"
        )
        .header(COOKIE, format!("session={}", session))
        .send()?
        .error_for_status()?;

    res.text()
}

