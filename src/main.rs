use colored::Colorize;
#[tokio::main]
async fn main() {

    let url = std::env::args().nth(1).expect("no url name given");
    let method = std::env::args().nth(2).expect("no method name given");


    let client = reqwest::Client::new();
    let mut res;


    match method.as_str() {
        "get" => {
            res = client.get(url).send();
            match res.await {
                Ok(res) => {
                    println!("{}: {}", "Status".red(), res.status());
                    print!("\n");
                    println!("--- {} ---", "Header".blue());
                    println!("{:#?}", res.headers());
                }
                Err(_) => {
                    println!("Error");
                }
            }
        },
        "post" => {
            res = client.post(url).send();
            match res.await {
                Ok(res) => {
                    println!("{}: {}", "Status".red(), res.status());
                    print!("\n");
                    println!("--- {} ---", "Header".blue());
                    println!("{:#?}", res.headers());
                }
                Err(_) => {
                    println!("Error");
                }
            }
        }
        _ => {
            println!("{}", "Erorr ...".blink());
        }
    }






}
