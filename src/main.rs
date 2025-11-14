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
fn read_user() ->  (Url, String, String) {
    let mut user_inp = String::new();
    io::stdin().read_line(&mut user_inp).expect("Some thing went wrong.");
    let parsed = Url::parse(user_inp.trim()).expect("Something went wrong ig");
    let base_url: String = parsed.host_str().expect("Something went wrong whilst parsing the url. Make sure that it is indeed a real, valid url.").to_string();
    return (parsed, base_url, user_inp); 

}



#[tokio::main]
async fn main() -> Result<()>{ 
    println!("Please enter the site you'd like to find the links in: ");
    let (parsed, base_url, user_inp) = read_user();
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


