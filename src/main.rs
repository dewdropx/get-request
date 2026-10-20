use error_chain::error_chain; // error handling
use std::io::Read; // standard io read

// handle errors
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn main() -> Result<()> {
    // store get info in response
    let mut response = reqwest::blocking::get("http://httpbin.org/get")?;

    // convert to string
    let mut body = String::new();
    response.read_to_string(&mut body)?;

    // print necessary info
    println!("Status: {}", response.status());
    println!("Headers:\n{:#?}", response.headers());
    println!("Body:\n{}", body);

    Ok(())
}