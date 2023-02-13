use std::{env};

use brocade::brocade::product::Product;

fn main() {
    let barcode = get_barcode_arg();
    if barcode.is_some() {
        match brocade::instance().get(&barcode.unwrap()) {
            Ok(info) => print_response(info),
            Err(err) => {
                eprintln!("{}", err);
            }
        }
    }
}

fn print_response(product: Product) {
    match serde_json::to_string_pretty(&product) {
        Ok(prod) => println!("{prod}"),
        Err(_) => println!("Invalid product data")
    }
}

fn get_barcode_arg() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        usage();
        return None;
    } else {
        return verify_barcode(args[1].clone());
    }
}

fn usage() {
    println!("{} barcode", get_program_name());
}

fn get_program_name() -> String {
    let args: Vec<String> = env::args().collect();
    String::from(args[0].clone())
}

fn verify_barcode(barcode: String) -> Option<String> {
    let number_of_non_digits = barcode.as_bytes().iter().filter(|c| {!c.is_ascii_digit()}).count();
    if number_of_non_digits != 0 {
        eprintln!("Invalid barcode");
        return None;
    } else {
        return Some(format!("{:#014}", barcode))
    }
}