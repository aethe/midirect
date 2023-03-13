use crate::{Config, GenericError};
use enigo::Key;
use std::collections::HashMap;

pub struct Mapping {
    pub notes: HashMap<u8, Key>,
    pub controls: HashMap<u8, Key>,
}

impl Mapping {
    pub fn from(config: &Config) -> Result<Self, GenericError> {
        Ok(Self {
            notes: Self::parse_config_hash_map(config.notes.as_ref().unwrap_or(&HashMap::new()))?,
            controls: Self::parse_config_hash_map(
                config.controls.as_ref().unwrap_or(&HashMap::new()),
            )?,
        })
    }

    fn parse_config_hash_map(
        map: &HashMap<String, String>,
    ) -> Result<HashMap<u8, Key>, GenericError> {
        map.iter().try_fold(
            HashMap::<u8, Key>::new(),
            |mut result, next| -> Result<HashMap<u8, Key>, GenericError> {
                result.insert(
                    next.0.parse::<u8>().map_err(|_| {
                        GenericError::new(format!("could not parse the note {}", next.0))
                    })?,
                    Self::parse_config_key(&next.1)?,
                );

                Ok(result)
            },
        )
    }

    fn parse_config_key(key: &str) -> Result<Key, GenericError> {
        match key {
            "alt" => Ok(Key::Alt),
            "backspace" => Ok(Key::Backspace),
            "caps_lock" => Ok(Key::CapsLock),
            "control" => Ok(Key::Control),
            "delete" => Ok(Key::Delete),
            "down_arrow" => Ok(Key::DownArrow),
            "end" => Ok(Key::End),
            "escape" => Ok(Key::Escape),
            "f1" => Ok(Key::F1),
            "f2" => Ok(Key::F2),
            "f3" => Ok(Key::F3),
            "f4" => Ok(Key::F4),
            "f5" => Ok(Key::F5),
            "f6" => Ok(Key::F6),
            "f7" => Ok(Key::F7),
            "f8" => Ok(Key::F8),
            "f9" => Ok(Key::F9),
            "f10" => Ok(Key::F10),
            "f11" => Ok(Key::F11),
            "f12" => Ok(Key::F12),
            "f13" => Ok(Key::F13),
            "f14" => Ok(Key::F14),
            "f15" => Ok(Key::F15),
            "f16" => Ok(Key::F16),
            "f17" => Ok(Key::F17),
            "f18" => Ok(Key::F18),
            "f19" => Ok(Key::F19),
            "f20" => Ok(Key::F20),
            "home" => Ok(Key::Home),
            "left_arrow" => Ok(Key::LeftArrow),
            "meta" => Ok(Key::Meta),
            "option" => Ok(Key::Option),
            "page_down" => Ok(Key::PageDown),
            "page_up" => Ok(Key::PageUp),
            "return" => Ok(Key::Return),
            "right_arrow" => Ok(Key::RightArrow),
            "shift" => Ok(Key::Shift),
            "space" => Ok(Key::Space),
            "tab" => Ok(Key::Tab),
            "up_arrow" => Ok(Key::UpArrow),
            key => {
                if key.chars().count() == 1 {
                    Ok(Key::Layout(key.chars().next().unwrap()))
                } else {
                    Err(GenericError::new(format!(
                        "{} is not a known key on a keyboard",
                        key
                    )))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mapping_from_config() {
        let mut notes = HashMap::<String, String>::new();
        notes.insert("60".to_string(), "a".to_string());
        notes.insert("61".to_string(), "meta".to_string());

        let mut controls = HashMap::<String, String>::new();
        controls.insert("62".to_string(), "b".to_string());
        controls.insert("63".to_string(), "shift".to_string());

        let config = Config::new(Some(notes), Some(controls));
        let mapping = Mapping::from(&config).unwrap();

        // test existing mappings
        assert_eq!(mapping.notes.get(&60), Some(&Key::Layout('a')));
        assert_eq!(mapping.notes.get(&61), Some(&Key::Meta));
        assert_eq!(mapping.controls.get(&62), Some(&Key::Layout('b')));
        assert_eq!(mapping.controls.get(&63), Some(&Key::Shift));

        // test missing mappings
        assert_eq!(mapping.notes.get(&64), None);
        assert_eq!(mapping.controls.get(&65), None);
    }

    #[test]
    fn mapping_from_config_with_missing_fields() {
        let config = Config::new(None, None);
        let mapping = Mapping::from(&config).unwrap();
        assert!(mapping.notes.is_empty());
        assert!(mapping.controls.is_empty());
    }
}
