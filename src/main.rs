pub mod lib;
pub mod endpoints;

fn main() {
    let mut arguments = std::env::args().skip(1);
    let flavour = arguments.next().expect("Version was not given.");
    let id = arguments.next().expect("Version was not given.");

    if flavour == "vanilla" {
        endpoints::vanilla::find_download(id.as_str());
    } else if flavour == "paper" {
        endpoints::paper::find_download(id.as_str())
    } 
}
