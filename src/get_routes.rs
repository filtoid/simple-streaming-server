use std::{ffi::OsStr, fs, path::Path};
use urlencoding;


use serde_json::json;
use warp::{filters::BoxedFilter, Filter, Rejection, Reply};
use warp_range::{filter_range, get_range};

pub(super) async fn make_routes(folder: &BoxedFilter<(String,)>) -> BoxedFilter<(impl Reply,)> {
    let default = warp::any().map(|| {
        let body: String = fs::read_to_string("templates/404.html").unwrap().parse().unwrap();
        warp::reply::html(
            body
        )
    });

    let home = warp::path::end().map(|| {
        let body: String = fs::read_to_string("templates/index.html").unwrap().parse().unwrap();
        // let mut handlebars = Handlebars::new();
        // handlebars.register_template_string("tpl_1", body).unwrap();
        warp::reply::html(
            body
        )
    });

    let get_file_list = warp::path!("api" / "files")
        .and(
            warp::filters::query::raw()
               .or(warp::any().map(|| String::default()))
               .unify()
            )
        .and(folder.clone())
        .and_then(get_file_list_impl);

    let get_video = warp::path!("video")
        .and(
            warp::filters::query::raw()
            .or(warp::any().map(|| String::default()))
            .unify()
        )
        .and(folder.clone())
        .and(filter_range())
        .and_then(get_video_impl);

    // GET /js/<file> - get named js file
    let get_js = warp::path("js").and(warp::fs::dir("./assets/js/"));
    // GET /css/<file> - get named css file
    let get_css = warp::path("css").and(warp::fs::dir("./assets/css/"));

    get_video
        .or(get_js)
        .or(get_css)
        .or(home)
        .or(get_file_list)
        .or(default)
        .boxed()
}

async fn get_file_list_impl(query_string: String, folder: String) -> Result<Box<dyn Reply>, Rejection> {
    let mut folders = Vec::<String>::new();
    let mut files = Vec::<String>::new();
    let query_string = urlencoding::decode(&query_string).unwrap().to_string();
    
    let path_str = format!("{}/{}", folder, query_string);
    
    let paths = match fs::read_dir(path_str.clone()){
        Ok(p) => p,
        Err(err)=> {
            log::error!("Error getting path {}: {}", path_str.clone(), err.to_string());
            return Ok(
                Box::new(
                    warp::reply::json(&json!({"status": "fail", "message": "Failed to find folder", "folders": [], "files": []}))
                )
            )
        }
    };

    for path in paths {
        match path {
            Ok(p) => match p.metadata() {
                Ok(md) => {
                    if md.is_dir() {
                        let out_str = format!("{:?}", p.file_name()).replace("\\", "/").replace("//",  "/");
                        folders.push(out_str);
                    } else if md.is_file() {
                        let out_str = format!("{:?}", p.file_name()).replace("\\", "/").replace("//",  "/");
                        let file_type = Path::new(out_str.clone().as_str())
                            .extension()
                            .and_then(OsStr::to_str)
                            .unwrap()
                            .to_string();
                        if file_type.to_lowercase() == "mp4"{
                            files.push(out_str);
                        }
                    } else {
                        log::error!("Path {:?} is not folder or file", p.path());
                    }
                },
                Err(_err) => {}
            },
            Err(_err) => {}
        }
    }

    return Ok(
        Box::new(
            warp::reply::json(&json!({"status": "ok", "message": "", "folders": folders, "files": files}))
        )
    )
}

async fn get_video_impl(query_string: String, folder: String, filter_range: Option<String>) -> Result<Box<dyn Reply>, Rejection> {
    let query_string = urlencoding::decode(&query_string).unwrap().to_string();
    let video_str = format!("{}/{}", folder, query_string);

    if !Path::new(video_str.clone().as_str()).exists() {
        return Ok(
            Box::new(
                warp::reply::json(&json!({"status": "fail", "message": "Failed to find video"}))
            )
        )
    }

    let file_type = Path::new(video_str.clone().as_str())
        .extension()
        .and_then(OsStr::to_str)
        .unwrap()
        .to_string();

    let mime_type = if file_type == "mkv" {
        "video/x-matroska"
    } else {
        "video/mp4"
    };

    Ok(
        Box::new(
            get_range(filter_range, video_str.as_str(), mime_type).await.unwrap()
        )
    )
}