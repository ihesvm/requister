use std::collections::HashMap;
use colored::Colorize;
use reqwest::{Response, StatusCode};
use serde_json;

async fn send_request(url: String, method: Option<String>, data: HashMap<String, String>) {
    let client = reqwest::Client::new();
    match method.as_ref().map(String::as_ref) {
        Some("get") | None => {
            get_method(client, url).await;
        }
        Some("delete") => {
            delete_method(client, url, data).await;
        },
        Some("post") => {
            post_method(client, url, data).await;
        },
        Some("put") => {
            put_method(client, url, data).await;
        },
        Some("patch") => {
            patch_method(client, url, data).await;
        }
        _ => {
            println!("{}", "Erorr ...".blink());
        }
    }
}


pub fn hash_collect(data: Vec<String>) -> HashMap<String, String> {
    let db: HashMap<String, String> = data
        .iter()
        .filter(|arg| arg.contains("="))
        .map(|arg| arg.split("=").collect::<Vec<&str>>())
        .map(|parts| (parts[0].to_string(), parts[1].to_string()))
        .collect();
    db
}

#[tokio::main]

async fn main() {

    

    let url = std::env::args().nth(1).expect("no url name given");
    let method = std::env::args().nth(2);
    let data: Vec<String> = std::env::args().collect();

    let data = hash_collect(data);


    
    send_request(url, method, data).await;

}


async fn delete_method(client: reqwest::Client, url: String, data: HashMap<String, String>) {
    let res = client.delete(url).json(&data).send().await;
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
            println!("--- {} ---", "Result".blue());
            Ok(println!("{:#?}", json))
        }
        StatusCode::METHOD_NOT_ALLOWED => {
            Ok(println!("{}", "Method Not Allowed!".red()))
        }
        StatusCode::UNAUTHORIZED => {
            Ok(println!("{}", "UNAUTHORIZED!".red()))
        },
        StatusCode::NOT_FOUND => {
            Ok(println!("{}", "Object Not Found ...".red()))
        },
        StatusCode::NO_CONTENT => {
            Ok(println!("{}", "No Content ...".red()))
        },
        StatusCode::BAD_REQUEST => {
            Ok(println!("{}", "Bad Request ...".red()))
        }

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
async fn post_method(client: reqwest::Client, url: String, data: HashMap<String, String>) {
    let res = client.post(url).json(&data).send().await;
    match res {
        Ok(res) => {
            response(res).await.unwrap();
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}


async fn put_method(client: reqwest::Client, url: String, data: HashMap<String, String>) {
    let res = client.put(url).json(&data).send().await;
    match res {
        Ok(res) => {
            response(res).await.unwrap();
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}

async fn patch_method(client: reqwest::Client, url: String, data: HashMap<String, String>) {
    let res = client.patch(url).json(&data).send().await;
    match res {
        Ok(res) => {
            response(res).await.unwrap();
        }
        Err(err) => {
            panic!("Error: {}", err);
        }
    }
}
