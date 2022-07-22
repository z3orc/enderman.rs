use std::collections::HashMap;
use std::fmt::format;
use std::hash::Hash;
use serde::de::value::MapDeserializer;
use serde::{Deserialize};
use serde_json::{Value};


#[derive(Deserialize, Debug)]
struct Version{
    id: String,
    // r#type: String,
    url: String,
}

#[derive(Deserialize, Debug)]
struct Project{
    project_id: String,
    project_name: String,
    version_groups: Vec<String>,
    versions: Vec<String>
}

#[derive(Deserialize, Debug)]
struct VersionPaper{
    // project_id: String,
    // project_name: String,
    // version: String,
    builds: Vec<u8>
}

#[derive(Deserialize, Debug)]
struct BuildPaper{
    downloads: BuildPaperDownloads
}

#[derive(Deserialize, Debug)]
struct BuildPaperDownloads{
    application: HashMap<String, String>,
}

fn main() {
    let mut arguments = std::env::args().skip(1);
    let flavour = arguments.next().expect("Version was not given.");
    let id = arguments.next().expect("Version was not given.");

    if flavour == "vanilla" {
        find_vanilla_download(id.as_str());
    } else if flavour == "paper" {
        find_paper_download(id.as_str());
    }

    
    
}

fn find_vanilla_download(id: &str){
    let raw_json = fetch_json("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json");
    let version_list = json_deserialize_mojang(raw_json);

    for item in version_list {
        if id == item.id {
            
            let build = fetch_json(&item.url);
            
            println!("{}", build["downloads"]["server"]["url"])
        }
    }
}

fn find_paper_download(id: &str){
    let response = reqwest::blocking::get("https://api.papermc.io/v2/projects/paper/").unwrap();
    let response_text = response.text().unwrap();
    let json: Project = serde_json::from_str(&response_text).expect("Could not parse json");
    let version_list = json.versions;
    

    for item in version_list {
        if id == item {
            let response = reqwest::blocking::get(format!("https://api.papermc.io/v2/projects/paper/versions/{}", id)).unwrap();
            let response_text = response.text().unwrap();
            let json: VersionPaper = serde_json::from_str(&response_text).expect("Could not parse json");
            let build = json.builds.last().unwrap();
            
            let response = reqwest::blocking::get(format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{:?}", id, build)).unwrap();
            let response_text = response.text().unwrap();

            let json: BuildPaper = serde_json::from_str(&response_text).expect("Could not parse json");

            let file = &json.downloads.application["name"];
            let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}", id, build, file);

            println!("{:?}", url);
        }
    }
}

fn json_deserialize_mojang(json: Value) -> Vec<Version>{
    let version_list = Vec::<Version>::deserialize(&json["versions"]).unwrap();
    return version_list
}

fn fetch_json(url: &str) -> Value{
    let response = reqwest::blocking::get(url).unwrap();
    let raw_json = response.text().unwrap();
    let raw_raw_json: Value = serde_json::from_str(&raw_json).expect("Could not parse json");
    return raw_raw_json
}

// fn fetch_url(version_id: &str, json_url: &str) -> Result<String, String>{
//     let json = fetch_json(json_url);
//     let version_list = json_deserialize_mojang(json);
//     for item in version_list {
//         if version_id == item.id {
//             print!("Found it!");
//             return Ok(item.url);
//         }
//     }
//     let errMessage = "Could not find version";
//     return Err(errMessage.to_string())
// }
