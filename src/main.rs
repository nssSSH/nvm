use url::{Url, Host, Position};
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
    println!("Please enter the site you'd like to find the links in: ");
    let mut user_inp = String::new();
    io::stdin().read_line(&mut user_inp).expect("Some shit went wrong!");
    let parsed = Url::parse(user_inp.trim()).expect("Something went wrong...");
    let base_url = parsed.host_str().expect("Something went wrong whilst parsing the url. Make sure that it is indeed a real, valid url.");
    println!("The base url is: {base_url}");
    let res = reqwest::get(user_inp)
        .await?
        .text()
        .await?;
    println!("The links within the sites are as follows: ");
    let domain_list: Vec<String> =Document::from(res.as_str())
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .map(|x| x.to_string())
        .collect();
    for link in &domain_list{
        println!("{link}");
    }
    
    Ok(())
}
/*
async fn fetch_urls(url_list: Vec<String>) {
        for i in url_list{
                    let res = reqwest::get(url_list[i])
                        .await?
                        .text()
                        .await?
  

  */
