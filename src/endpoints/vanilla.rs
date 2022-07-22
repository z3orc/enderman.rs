use serde_json::{Value};
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct Version{
    id: String,
    // r#type: String,
    url: String,
}

pub fn find_download(id: &str){
    let raw_json = crate::lib::fetch_json("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json");
    let version_list = json_deserialize(raw_json);

    for item in version_list {
        if id == item.id {
            
            let build = crate::lib::fetch_json(&item.url);
            
            println!("{}", build["downloads"]["server"]["url"])
        }
    }
}

fn json_deserialize(json: Value) -> Vec<Version>{
    let version_list = Vec::<Version>::deserialize(&json["versions"]).unwrap();
    return version_list
}