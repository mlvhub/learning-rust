extern crate reqwest;
extern crate select;
extern crate url;

use std::collections::HashSet;

use url::{Url, Position, ParseError};
use reqwest::StatusCode;
use select::document::Document;
use select::predicate::Name;

fn get_base_url(url: &Url, doc: &Document) -> Result<Url, ParseError> {
    let base_tag_href = doc.find(Name("base")).filter_map(|n| n.attr("href")).nth(0);

    let base_url = base_tag_href.map_or_else(
        || Url::parse(&url[..Position::BeforePath]),
        Url::parse,
    )?;

    Ok(base_url)
}

fn check_link(url: &Url) -> reqwest::Result<bool> {
    let res = reqwest::get(url.as_ref())?;

    Ok(res.status() != StatusCode::NotFound)
}

fn main() {
    let url = Url::parse("https://www.rust-lang.org/en-US/").unwrap();

    let res = reqwest::get(url.as_ref()).unwrap();
    let document = Document::from_read(res).unwrap();

    let base_url = get_base_url(&url, &document).unwrap();

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
}
