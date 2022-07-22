use std::{collections::HashMap, hash::Hash};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Project{
    // project_id: String,
    // project_name: String,
    // version_groups: Vec<String>,
    versions: Vec<String>
}

#[derive(Deserialize, Debug)]
struct Version{
    // project_id: String,
    // project_name: String,
    // version: String,
    builds: Vec<u16>
}

#[derive(Deserialize, Debug)]
struct Build{
    downloads: HashMap<String, HashMap<String,String>>
}

pub fn find_download(id: &str){
    let response = reqwest::blocking::get("https://api.papermc.io/v2/projects/paper/").unwrap();
    let response_text = response.text().unwrap();
    let json: Project = serde_json::from_str(&response_text).expect("Could not parse json");
    let version_list = json.versions;
    

    for item in version_list {
        if id == item {
            let response = reqwest::blocking::get(format!("https://api.papermc.io/v2/projects/paper/versions/{}", id)).unwrap();
            let response_text = response.text().unwrap();
            let json: Version = serde_json::from_str(&response_text).expect("Could not parse json");
            let build = json.builds.last().unwrap();
            
            let response = reqwest::blocking::get(format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{:?}", id, build)).unwrap();
            let response_text = response.text().unwrap();

            let json: Build = serde_json::from_str(&response_text).expect("Could not parse json");

            let file = &json.downloads["application"]["name"];
            let url = format!("https://api.papermc.io/v2/projects/paper/versions/{}/builds/{}/downloads/{}", id, build, file);

            println!("{:?}", url);
        }
    }
}