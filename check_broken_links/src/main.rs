extern crate reqwest;
extern crate select;
extern crate url;

use std::collections::HashSet;

use url::{Url, Position};
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;

fn get_base_url(url: &Url, doc: &Document) -> Result<Url, url::ParseError> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url = base_tag_href.map_or_else(
        || Url::parse(&url[..Position::BeforePath]),
        Url::parse,
    )?;

    Ok(base_url)
}

fn check_link(url: &Url) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();
    // TODO: benchmark GET vs HEAD
    let res = client.head(url.as_ref()).send()?;

    Ok(res.status() != StatusCode::NotFound)
}

#[derive(Debug)]
enum AppError {
    UrlParseError(url::ParseError),
    IOError(std::io::Error),
    RequestError(reqwest::Error),
}

impl From<url::ParseError> for AppError {
    fn from(error: url::ParseError) -> Self {
        AppError::UrlParseError(error)
    }
}
impl From<std::io::Error> for AppError {
    fn from(error: std::io::Error) -> Self {
        AppError::IOError(error)
    }
}
impl From<reqwest::Error> for AppError {
    fn from(error: reqwest::Error) -> Self {
        AppError::RequestError(error)
    }
}

fn run() -> Result<(), AppError> {
    //    Commented to showcase the manual alternative
    //    let url = Url::parse("https://www.rust-lang.org/en-US/")?;
    let url = match Url::parse("https://www.rust-lang.org/en-US/") {
        Ok(url) => url,
        Err(error) => return Err(AppError::UrlParseError(error))
    };

    let res = reqwest::get(url.as_ref())?;
    let document = Document::from_read(res)?;

    let base_url = get_base_url(&url, &document)?;

    let base_parser = Url::options().base_url(Some(&base_url));

    let links: HashSet<Url> = document
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .filter_map(|link| base_parser.parse(link).ok())
        .collect();

    links
        .iter()
        .filter(|link| check_link(link).ok() == Some(false))
        .for_each(|x| println!("{} is broken.", x));

    Ok(())
}

fn main() {
    run().unwrap();
}
