use serde::Deserialize;

use serde_json::from_reader;

use std::{env, fs::File, io::Write, process};

#[derive(Debug, Deserialize)]
struct TiledMap {
    height: usize,
    width: usize,
    layers: Vec<TiledLayer>,
}

#[derive(Debug, Deserialize)]
struct TiledLayer {
    data: Vec<usize>,
}

fn get_solid_list<'a>(map: &'a TiledMap) -> impl Iterator<Item = (usize, usize)> + 'a {
    map.layers
        .iter()
        .nth(1)
        .expect("Map has only one layer")
        .data
        .iter()
        .enumerate()
        .filter(|(_, tile)| **tile != 0)
        .map(move |(index, _)| (index / map.width, index % map.width))
}

fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        Some(map_file) => {
            let map: TiledMap = {
                let file = File::open(map_file).expect("Failed to open map file");
                from_reader(file).expect("Failed to deserialize map")
            };

            let output = format!(
                "(\n\tnum_tiles_x: {},\n\tnum_tiles_y: {},\n\tsolids: [\n{}\t],\n)\n",
                map.width,
                map.height,
                get_solid_list(&map)
                    .map(|(x, y)| format!("\t\t[{}, {}],\n", x, y))
                    .collect::<Vec<String>>()
                    .join("")
            );

            let output_file_name = "out.txt";

            File::create(output_file_name)
                .expect("Failed to open output file")
                .write(output.as_bytes())
                .expect("Failed to write to output file");

            println!("Saved output to {}.", output_file_name);
        },
        None => {
            println!("Usage: map_parser tiled_map.json");
            process::exit(1);
        },
    }
}
