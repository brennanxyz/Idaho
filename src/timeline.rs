use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::{env, io::Read};
use tracing::{event, Level};

use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GameKeyCode(KeyCode);

impl<'de> Deserialize<'de> for GameKeyCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_uppercase().as_str() {
            "W" => Ok(GameKeyCode(KeyCode::KeyW)),
            "A" => Ok(GameKeyCode(KeyCode::KeyA)),
            "S" => Ok(GameKeyCode(KeyCode::KeyS)),
            "D" => Ok(GameKeyCode(KeyCode::KeyD)),
            "Q" => Ok(GameKeyCode(KeyCode::KeyQ)),
            "E" => Ok(GameKeyCode(KeyCode::KeyE)),
            "R" => Ok(GameKeyCode(KeyCode::KeyR)),
            "T" => Ok(GameKeyCode(KeyCode::KeyT)),
            "Y" => Ok(GameKeyCode(KeyCode::KeyY)),
            "N" => Ok(GameKeyCode(KeyCode::KeyN)),
            "M" => Ok(GameKeyCode(KeyCode::KeyM)),
            "F" => Ok(GameKeyCode(KeyCode::KeyF)),
            "G" => Ok(GameKeyCode(KeyCode::KeyG)),
            "H" => Ok(GameKeyCode(KeyCode::KeyH)),
            "Z" => Ok(GameKeyCode(KeyCode::KeyZ)),
            "X" => Ok(GameKeyCode(KeyCode::KeyX)),
            "C" => Ok(GameKeyCode(KeyCode::KeyC)),
            "SPACE" => Ok(GameKeyCode(KeyCode::Space)),
            "ENTER" => Ok(GameKeyCode(KeyCode::Enter)),
            _ => Err(serde::de::Error::custom(format!("Invalid key: {}", s))),
        }
    }
}

impl Serialize for GameKeyCode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let key_str = match self.0 {
            KeyCode::KeyW => "W",
            KeyCode::KeyA => "A", 
            KeyCode::KeyS => "S",
            KeyCode::KeyD => "D",
            KeyCode::KeyQ => "Q",
            KeyCode::KeyE => "E",
            KeyCode::KeyR => "R",
            KeyCode::KeyT => "T",
            KeyCode::KeyY => "Y",
            KeyCode::KeyN => "N",
            KeyCode::KeyM => "M",
            KeyCode::KeyF => "F",
            KeyCode::KeyG => "G",
            KeyCode::KeyH => "H",
            KeyCode::KeyZ => "Z",
            KeyCode::KeyX => "X",
            KeyCode::KeyC => "C",
            KeyCode::Space => "SPACE",
            KeyCode::Enter => "ENTER",
            _ => return Err(serde::ser::Error::custom(format!("Invalid key: {:?}", self.0))),
        };
        serializer.serialize_str(key_str)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub enum Trigger {
    #[serde(rename = "complete_action")]
    CompleteAction(String),
    #[serde(rename = "character_interaction")]
    CharacterInteraction(String),
    #[serde(rename = "itme_interaction")]
    ItemInteraction(String),
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Response {
    pub text: String,
    pub key: GameKeyCode,
}

#[derive(Clone, Debug, Eq, PartialEq, Deserialize)]
pub struct Action {
    pub name: String,
    pub trigger: Trigger,
    pub depends_on: Option<String>,
    pub text: Option<String>,
    #[serde(rename = "available_response")]
    pub available_responses: Vec<Response>,
    pub given_response: Option<Response>,
    pub acknowledged: bool,
    pub completed: bool,
}

#[derive(Asset, Clone, Component, Debug, Eq, Default, PartialEq, Deserialize, TypePath)]
pub struct Timeline {
    #[serde(rename = "action")]
    pub actions: Vec<Action>
}

impl Timeline {
    pub fn from(entity_instance: &EntityInstance) -> Self {
        // attempt to load the timeline file for the entity instance
        event!(Level::INFO, "Loading timeline file");
        let timeline_filename = match LdtkFields::get_string_field(entity_instance, "timeline") {
            Ok(filename) => filename,
            Err(_) => {
                event!(Level::ERROR, "No timeline file set");
                return Timeline { actions: vec![] };
            },
        };
        
        dotenv().ok();

        let asset_folder = match env::var("ASSET_BASE_DIR") {
            Ok(folder) => folder,
            Err(_) => {
                event!(Level::ERROR, "Error loading environment variable");
                return Timeline { actions: vec![] };
            }
        };

        // get xml file from std::fs
        let mut file = match std::fs::File::open(format!("{}/{}/{}", asset_folder, "timelines", timeline_filename)) {
            Ok(file) => {
                event!(Level::INFO, "Loaded timeline file | {}", timeline_filename);
                file
            },
            Err(_) => {
                event!(Level::WARN, "Error opening file. Attempting init");
                match std::fs::File::open(format!("{}/{}/{}", asset_folder, "timelines", timeline_filename.replace(".xml", "_init.xml"))) {
                    Ok(file) => {
                        event!(Level::INFO, "Loaded init timeline file | {}", timeline_filename.replace(".xml", "_init.xml"));
                        file
                    },
                    Err(_) => {
                        event!(Level::ERROR, "Error opening file.");
                        return Timeline { actions: vec![] };
                    }
                }
            }
        };

        let mut contents = String::new();

        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => {
                event!(Level::ERROR, "Error reading file");
                return Timeline { actions: vec![] };
            }
        }

        let timeline: Timeline = match serde_xml_rs::from_str(&contents) {
            Ok(timeline) => timeline,
            Err(e) => {
                event!(Level::ERROR, "Error parsing file | {}", e);
                Timeline { actions: vec![] }
            }
        };

        timeline
    }
}
