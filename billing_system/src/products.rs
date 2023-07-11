use crate::utils::ProductInfo;
use std::error::Error;

fn read_from_file(path: &str) ->  Result<(), Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(path)?;

    let headers = reader.headers()?;
    println!("{} {}", headers.get(0).unwrap_or("-"), headers.get(1).unwrap_or("-"));

    for result in reader.deserialize() {
        let record: ProductInfo = result?;
        println!("{}", record);
    }

    Ok(())
}

pub fn view_products(){
    if let Err(err) = read_from_file("./data/products.csv") {
        eprintln!("{}", err);
    }
}
