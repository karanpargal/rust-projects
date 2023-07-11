use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct ProductInfo {
    name: String,
    rate: u32
}

fn read_from_file(path: &str) ->  Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers();
    println!("{:?}", headers);

    for result in reader.deserialize() {
        let record: ProductInfo = result?;
        println!("{:?}", record);
    }

    Ok(())
}

pub fn view_products(){
    if let Err(err) = read_from_file("./data/products.csv") {
        eprintln!("{}", err);
    }
}
