
use serde_json::Value;
use std::fs;
use std::path::Path;
use wot_replay_parser::ReplayParser;

pub fn main() {
    let folder_path = "/home/zray/Desktop/replays";  
    
    // Read all files in the directory
    let mut i = 0;
    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if i == 85 {
                    return;
                }
                let path = entry.path();
                // Check if it's a .wotreplay file
                if path.extension().and_then(|s| s.to_str()) == Some("wotreplay") {
                    match process_replay(&path) {
                        Ok(damage) => {
                            if damage != 0 {
                                println!("{}", damage);
                                    i+= 1;
                            }
                        },
                        Err(e) => println!("Error processing {}: {}", path.display(), e),
                    }
                }
            }
        }
    }
}

fn process_replay(path: &Path) -> Result<i32, Box<dyn std::error::Error>> {
    let replay_parser = ReplayParser::parse_file(path)?;
    let replay_json_end = replay_parser.replay_json_end();
    let json_string_end = serde_json::to_string_pretty(&replay_json_end)?;
    Ok(get_damage(&json_string_end))
}

fn get_damage(json_str: &str) -> i32 {
    let data: Value = serde_json::from_str(json_str).unwrap();
    
    // Get the first object in the array
    if let Some(first_obj) = data.as_array().and_then(|arr| arr.first()) {
        // Navigate to personal -> first key -> damageDealt
        if let Some(personal) = first_obj.get("personal") {
            if let Some((_key, player_data)) = personal.as_object().and_then(|obj| obj.iter().next()) {
                if let Some(damage) = player_data.get("damageDealt") {
                    return damage.as_i64().unwrap_or(0) as i32;
                }
            }
        }
    }
    0
}
