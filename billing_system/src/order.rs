use inquire::{Select, CustomType};
use std::error::Error;
use crate::utils::ProductInfo;

fn generate_options(path: &str) -> Result<Vec<ProductInfo>, Box<dyn Error>> {
    
    let mut reader = csv::Reader::from_path(path)?;
    let mut records: Vec<ProductInfo> = vec![];

    for result in reader.deserialize() {
        let record: ProductInfo = result.unwrap_or(ProductInfo::new());
        records.push(record);
    }

    Ok(records)
}

fn take_order() -> Result<Option<u64>, Box<dyn Error>> {
    let options = generate_options("./data/products.csv")?;
    let mut total: u64 = 0;

    loop {
        let product = Select::new("Select a product:", options.clone()).prompt()?;

        let unit = CustomType::<u64>::new("Units: ").prompt()?;
        total = total + (product.rate * unit);

        let next = Select::new("Add another product?", vec!["Yes", "No"]).prompt()?;
        if next == "No" {
            return Ok(Some(total));
        }
    }

}

pub fn place_order(){

    let order = take_order();

    match order {
        Ok(amount) => {
            if let Some(a) = amount {
                println!("Total amount: ${}", a);
            };
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
