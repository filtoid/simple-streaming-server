use std::fs;

use warp::Filter;
use toml::Table;

use serde::{Deserialize, Serialize};

mod get_routes;

#[derive(Serialize,Deserialize)]
struct Config {
    folders: Vec::<String>
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let conf_str = match fs::read_to_string("./config.toml"){
        Ok(res)=> res,
        Err(_err) => {
            println!("Error occured unable to find config");
            return
        }
    };

    let config = conf_str.parse::<Table>().unwrap();
    let folder: String = config["folders"][0].to_string().replace("\"", "");
    log::info!("Using root folder: {:?}", folder);
     
    let port: u16 = config["port"].as_integer().unwrap() as u16;

    log::info!("Application starting on port {}", port);
    
    let boxed_folder = warp::any().map(move || folder.clone()).boxed();
    let routes = get_routes::make_routes(&boxed_folder).await;
 
    warp::serve(routes)
        .run(([0, 0, 0, 0], port))
        .await;
}