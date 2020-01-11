// Make sure we have our third-party dependencies.
// (This is going away in future Rust, since it
// simply duplicates what's already in Cargo.toml.)
extern crate reqwest;
extern crate failure;

extern crate csv;

// Need to import a couple of things from
// the standard library
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

// Importing this allows us to autoderive
// the serialization traits.
#[macro_use]
extern crate serde_derive;

// This is where we get the serde traits from.
extern crate serde;

// An implementation of the serde encoders/decoders
// to and from a JSON. We'll need
// these later.
extern crate serde_json;

#[derive(Debug, Serialize, Deserialize)]
struct WishlistEntry {
    user_id: usize,
    book_id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Book {
    book_id: usize,
    title: String
}

/// Download file from `url` and save it to `destination`.
fn download(url: &str, destination: &Path)
    -> Result<(), failure::Error> {

        // Don't do anything if we already have the file.
        if destination.exists() {
            return Ok(())
        }

        // Otherwise, create a new file.

        // Because each of the following operations
        // can fail (returns a result type), we follow
        // them with the `?` operator. If the result
        // is an error, it will exit from the function
        // early, propagating the error upwards; if
        // the operation completed successfully, we get
        // the result instead.
        let file = File::create(destination)?;

        // We need the `mut` annotation, because
        // we're mutating (writing to) the writer.
        let mut writer = BufWriter::new(file);

        let mut response = reqwest::get(url)?;
        response.copy_to(&mut writer)?;

        Ok(())
    }

/// Download ratings and metadata both.
fn download_data(ratings_path: &Path, books_path: &Path) {
    let ratings_url = "https://github.com/zygmuntz/\
                       goodbooks-10k/raw/master/ratings.csv";
    let books_url = "https://github.com/zygmuntz/\
                     goodbooks-10k/raw/master/books.csv";

    download(&ratings_url,
             ratings_path).expect("Could not download ratings");
    download(&books_url,
             books_path).expect("Could not download metadata");
}

/// Deserialize from file at `path` into a vector of
/// `WishlistEntry`.
fn deserialize_ratings(path: &Path)
    -> Result<Vec<WishlistEntry>, failure::Error> {

        let mut reader = csv::Reader::from_path(path)?;

        // We specify the type of the deserialized entity
        // via a type annotation. Otherwise, the compiler has
        // no way of knowing what sort of thing we want to
        // deserialize!
        // We also do a further trick where instead of deserializing
        // into a vector of results, we deserialize into a result with
        // a vector.
        let entries = reader.deserialize()
            .collect::<Result<Vec<_>, _>>()?;

        Ok(entries)
    }

// We'll use the stdlib hashmap for the mapping.
use std::collections::HashMap;

/// Deserialize from file at `path` into the book
/// mappings.
fn deserialize_books(path: &Path)
    -> Result<(HashMap<usize, String>,
               HashMap<String, usize>), failure::Error> {

        let mut reader = csv::Reader::from_path(path)?;

        let entries: Vec<Book> = reader.deserialize::<Book>()
            .collect::<Result<Vec<_>, _>>()?;

        // We can simply iterate over the entries and collect
        // them into a different data structure. This is not
        // the most efficient solution but it will do for now.
        let id_to_title: HashMap<usize, String> = entries
            .iter()
            .map(|book| (book.book_id, book.title.clone()))
            .collect();
        let title_to_id: HashMap<String, usize> = entries
            .iter()
            .map(|book| (book.title.clone(), book.book_id))
            .collect();

        Ok((id_to_title, title_to_id))
    }

extern crate sbr;

use sbr::models::ewma::{Hyperparameters, ImplicitEWMAModel};
use sbr::models::{Loss, Optimizer};

fn build_model(num_items: usize) -> ImplicitEWMAModel {
    let hyperparameters = Hyperparameters::new(num_items, 128)
        .embedding_dim(32)
        .learning_rate(0.16)
        .l2_penalty(0.0004)
        .loss(Loss::WARP)
        .optimizer(Optimizer::Adagrad)
        .num_epochs(10)
        .num_threads(1);

    hyperparameters.build()
}

use sbr::data::{Interaction, Interactions};

fn build_interactions(data: &[WishlistEntry]) -> Interactions {
    // If the collection is empty, `max` doesn't exist. This
    // is why we get an Option back, which we then unwrap.
    let num_users = data
        .iter()
        .map(|x| x.user_id)
        .max()
        .unwrap() + 1;
    let num_items = data
        .iter()
        .map(|x| x.book_id)
        .max()
        .unwrap() + 1;

    let mut interactions = Interactions::new(num_users,
                                             num_items);

    // There are no timestamps in the interaction data, but
    // we make use of the fact that they are sorted by time.
    for (idx, datum) in data.iter().enumerate() {
        interactions.push(
            Interaction::new(datum.user_id,
                             datum.book_id,
                             idx)
        );
    }

    interactions
}

// We need to import the rand crate.
extern crate rand;
use rand::SeedableRng;

// We perform a split where the train and test
// sets are disjoint on the user dimension: no
// single user is in both.
use sbr::data::user_based_split;
use sbr::OnlineRankingModel;

use sbr::evaluation::mrr_score;

/// Fit the model.
///
/// If successful, return the MRR on the test set.
/// Otherwise, return an error.
fn fit(model: &mut ImplicitEWMAModel,
       data: &Interactions)
       -> Result<f32, failure::Error> {

    // Use a fixed seed for repeatable results.
    let mut rng = rand::XorShiftRng::from_seed([42; 16]);

    let (train, test) = user_based_split(data,
                                         &mut rng,
                                         0.2);

    model.fit(&train.to_compressed())?;

    let mrr = mrr_score(model, &test.to_compressed())?;

    Ok(mrr)
       }

fn serialize_model(model: &ImplicitEWMAModel,
                   path: &Path) -> Result<(), failure::Error> {

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);

    Ok(serde_json::to_writer(&mut writer, model)?)
}

/// Download training data and build a model.
///
/// We'll use this function to power the `fit`
/// subcommand of our command line tool.
fn main_build() {

    let ratings_path = Path::new("ratings.csv");
    let books_path = Path::new("books.csv");
    let model_path = Path::new("model.json");

    // Exit early if we already have a model.
    if model_path.exists() {
        println!("Model already fitted.");
        return ();
    }

    download_data(ratings_path, books_path);

    let ratings = deserialize_ratings(ratings_path).unwrap();
    let (id_to_title,
         title_to_id) = deserialize_books(books_path).unwrap();

    println!("Deserialized {} ratings.", ratings.len());
    println!("Deserialized {} books.", id_to_title.len());

    let interactions = build_interactions(&ratings);
    let mut model = build_model(interactions.num_items());

    println!("Fitting...");
    let mrr = fit(&mut model, &interactions)
        .expect("Unable to fit model.");
    println!("Fit model with MRR of {:.2}", mrr);

    serialize_model(&model, &model_path)
        .expect("Unable to serialize model.");
}

fn main() {
    println!("Hello, world!");
}
