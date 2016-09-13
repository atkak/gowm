use std::error::Error;
use std::result::Result;
use hyper::client::Client;
use hyper::header::UserAgent;
use hyper::status::StatusCode;
use rustc_serialize::json;

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Repo {
    name: String,
    html_url: String,
    git_url: String,
}

pub fn fetch_org_repos(org: &str) -> Result<Vec<Repo>, Box<Error>> {
    let client = Client::new();
    let ref url = format!("https://api.github.com/orgs/{}/repos", org);
    let mut res = client.get(url)
        .header(UserAgent("gowm".to_owned()))
        .send()
        .unwrap();

    let ref body = try!(
        match res.status {
            StatusCode::Ok => {
                let mut body = String::new();
                use std::io::Read;
                res.read_to_string(&mut body);
                Ok(body)
            },
            StatusCode::NotFound => Err(format!("Organization not found. name: {}", org)),
            _ => Err("Fail to access to GitHub".to_owned())
        }
    );

    let repos: Vec<Repo> = try!(
        json::decode(body).map_err(|err| err.description().to_owned())
    );

    Ok(repos)
}
