use error_chain::error_chain;
use select::document::Document;
use select::predicate::Name;
use std::io;
error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(std::io::Error);
    }
}

#[tokio::main]

async fn main() -> Result<()>{
    println!("Please enter the site you'd like to scrape: ");
    let mut user_inp = String::new();
    io::stdin().read_line(&mut user_inp).expect("Some shit went wrong!");
    let res = reqwest::get(user_inp)
        .await?
        .text()
        .await?;
    println!("The links within the sites are as follows: ");
    Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|x| println!("{x}"));

    Ok(())
}
