use colored::Colorize;
use reqwest::{Response, StatusCode};
use serde_json;


async fn send_request(url: String, method: Option<String>) {
    let client = reqwest::Client::new();
    match method.as_ref().map(String::as_ref) {
        Some("GET") | None => {
            get_method(client, url).await;
        },
        Some("DELETE") => {
            delete_method(client, url).await;
        }
        _ => {
            println!("{}", "Erorr ...".blink());
        }
    }
}



#[tokio::main]
async fn main() {
    let url = std::env::args().nth(1).expect("no url name given");
    let method = std::env::args().nth(2);
    send_request(url, method).await;

}



async fn delete_method(client: reqwest::Client, url: String) {
    let res = client.delete(url).send().await;
    match res {
        Ok(res) => {
            response(res).await.unwrap();
        }
        Err(err) => {
            println!("{}", err.status().unwrap().as_str());
        }
    }
}



async fn response(res: Response) -> Result<(), Box<dyn std::error::Error>> {
    match res.status() {
        StatusCode::OK => {
            println!("{}", "Success!".green());
            print!("\n");
            let json: serde_json::Value = res.json().await?;
            Ok(println!("{:#?}", json))
        },
        StatusCode::METHOD_NOT_ALLOWED => {
            Ok(println!("{}", "Method Not Allowed!".red()))
        },
        StatusCode::UNAUTHORIZED => {
            Ok(println!("{}", "UNAUTHORIZED!".red()))
        },

        _ => {
            panic!("Uh no! Something unexpected happened.");
        }
    }
}
async fn get_method(client: reqwest::Client, url: String) {
    let res = client.get(url).send().await;
    match res {
        Ok(res) => {
            response(res).await.unwrap();
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}