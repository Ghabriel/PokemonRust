use regex::{Error as RegexError, Regex};

use serde::Deserialize;

use serde_xml_rs::{Error as XmlError, from_reader};

use std::{
    env,
    fmt::{self, Display, Formatter},
    fs::{File, read_to_string},
    io::{Error as IoError, Write},
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

enum Error {
    Io(IoError),
    Regex(RegexError),
    Parsing(&'static str),
    Xml(XmlError),
}

impl From<IoError> for Error {
    fn from(error: IoError) -> Error {
        Error::Io(error)
    }
}

impl From<RegexError> for Error {
    fn from(error: RegexError) -> Error {
        Error::Regex(error)
    }
}

impl From<XmlError> for Error {
    fn from(error: XmlError) -> Error {
        Error::Xml(error)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(error) => write!(f, "{}", error),
            Error::Regex(error) => write!(f, "{}", error),
            Error::Parsing(error) => write!(f, "{}", error),
            Error::Xml(error) => write!(f, "{}", error),
        }
    }
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

fn update_map(map_folder_path: &Path) -> Result<(), Error> {
    let map_ron_path = map_folder_path.join("map.ron");

    let mut map_ron_content = read_to_string(&map_ron_path)?;

    let solid_list_range = {
        let regex = Regex::new(r"solids: \[[^\[]+\]")?;

        match regex.find(&map_ron_content) {
            Some(match_position) => {
                let start = match_position.start();
                let end = match_position.end();

                start..end
            },
            None => {
                return Err(Error::Parsing("Failed to find the list of solids in the map file."));
            },
        }
    };

    let map: TiledMap = {
        let map_tmx_file = map_folder_path.join("map.tmx");
        let file = File::open(map_tmx_file)?;
        from_reader(file)?
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

    File::create(&map_ron_path)?
        .write(map_ron_content.as_bytes())?;

    Ok(())
}

fn main() {
    let mut args = env::args().skip(1);

    match args.next() {
        Some(map_folder) => {
            let map_folder_path = Path::new(&map_folder);

            let result = update_map(&map_folder_path);

            match result {
                Ok(_) => {
                    let folder_name = map_folder_path
                        .file_name()
                        .expect("Invalid folder path")
                        .to_str()
                        .expect("Invalid folder name");

                    println!("Updated map \"{}\".", folder_name);
                },
                Err(err) => {
                    eprintln!("An error occurred: {}", err);
                    process::exit(2);
                },
            }
        },
        None => {
            println!("Usage: map_parser tiled_map.json");
            process::exit(1);
        },
    }
}
