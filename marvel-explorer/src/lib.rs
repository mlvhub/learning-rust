extern crate crypto;
extern crate hyper;
extern crate url;

use std::cell::RefCell;
use std::time::{SystemTime, UNIX_EPOCH};
use crypto::digest::Digest;
use crypto::md5::Md5;
use hyper::Uri;
use url::Url;

struct UriMaker {
    /// Our Marvel API *public* key
    key: String,
    /// Our Marvel API *private* key
    secret: String,
    /// The prefix of every url we'll be producing.
    api_base: String,
    /// Our md5 hasher, used to generate our `hash` query
    /// string parameter.
    hasher: RefCell<Md5>,
}

impl UriMaker {
    /// convenience method to initialize a new `UriMaker`.
    pub fn new(
        key: String,
        secret: String,
        api_base: String
    ) -> UriMaker {
        UriMaker {
            key,
            secret,
            api_base,
            hasher: RefCell::new(Md5::new()),
        }
    }

    /// Produces an md5 digest hash for ts + private key + public key
    fn get_hash(&self, ts: &str) -> String {
        // The `RefCell` lets us get a mutable reference to the
        // object within while not having to flag the whole `UriMaker`
        // as mutable.
        let mut hasher = self.hasher.borrow_mut();
        hasher.reset();
        hasher.input_str(ts);
        hasher.input_str(&self.secret);
        hasher.input_str(&self.key);
        hasher.result_str()
    }

    /// Convert from a `url::Url` to a `hyper::Uri`.
    fn url_to_uri(url: &url::Url) -> Uri {
        url.as_str().parse().unwrap()
    }

    /// Append a path to the api root, and set the authorization
    /// query string params.
    fn build_url(
        &self,
        path: &str
    ) -> Result<Url, url::ParseError> {
        let ts = {
            let since_the_epoch =
                SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let ms = since_the_epoch.as_secs() * 1000
                + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
            format!("{}", ms)
        };
        let hash = &self.get_hash(&ts);
        let mut url = Url::parse(&self.api_base)?.join(path)?;

        url.query_pairs_mut()
            .append_pair("ts", &ts)
            .append_pair("hash", hash)
            .append_pair("apikey", &self.key);
        Ok(url)
    }
    // ... snip ...
}

