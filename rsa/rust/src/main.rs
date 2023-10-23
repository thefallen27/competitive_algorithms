use std::error::Error;

mod openssl;
mod simple;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <simple|openssl>", args[0]);
        return Ok(());
    }

    match args[1].as_str() {
        "simple" => simple::run()?,
        "openssl" => openssl::run()?,
        _ => println!("Usage: {} <simple|openssl>", args[0]),
    };

    Ok(())
}
