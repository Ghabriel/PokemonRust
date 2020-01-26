use regex::Regex;

use serde::Deserialize;

use serde_xml_rs::from_reader;

use std::{
    env,
    fs::{File, read_to_string},
    io::Write,
    path::Path,
    process,
};

#[derive(Debug, Deserialize)]
struct TiledMap {
    height: usize,
    width: usize,

    #[serde(rename = "layer")]
    layers: Vec<TiledLayer>,
}

#[derive(Debug, Deserialize)]
struct TiledLayer {
    data: TiledLayerData,
}

#[derive(Debug, Deserialize)]
struct TiledLayerData {
    #[serde(rename = "$value")]
    body: String,
}

fn get_solid_list<'a>(map: &'a TiledMap) -> impl Iterator<Item = (usize, usize)> + 'a {
    map.layers
        .iter()
        .nth(1)
        .expect("Map has only one layer")
        .data
        .body
        .split(",")
        .map(|tile| tile.replace("\n", ""))
        .enumerate()
        .filter(|(_, tile)| *tile != "0")
        .map(move |(index, _)| (index % map.width, index / map.width))
        .map(move |(x, y)| (x, map.height - 1 - y))
}

fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        Some(map_folder) => {
            let map_folder_path = Path::new(&map_folder);
            let map_ron_path = map_folder_path.join("map.ron");

            let mut map_ron_content = read_to_string(&map_ron_path)
                .expect("Failed to open ron file for reading");

            let solid_list_range = {
                let regex = Regex::new(r"solids: \[[^\[]+\]").unwrap();
                let match_position = regex.find(&map_ron_content).unwrap();
                let start = match_position.start();
                let end = match_position.end();

                start..end
            };

            let map: TiledMap = {
                let map_tmx_file = map_folder_path.join("map.tmx");
                let file = File::open(map_tmx_file).expect("Failed to open map file");
                from_reader(file).expect("Failed to deserialize map")
            };

            map_ron_content.replace_range(
                solid_list_range,
                &format!(
                    "solids: [\n{}    ]",
                    get_solid_list(&map)
                        .map(|(x, y)| format!("        ({}, {}),\n", x, y))
                        .collect::<Vec<String>>()
                        .join(""),
                ),
            );

            File::create(&map_ron_path)
                .expect("Failed to open ron file for writing")
                .write(map_ron_content.as_bytes())
                .expect("Failed to write to ron file");

            let folder_name = map_folder_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();
            println!("Updated map \"{}\".", folder_name);
        },
        None => {
            println!("Usage: map_parser tiled_map.json");
            process::exit(1);
        },
    }
}
