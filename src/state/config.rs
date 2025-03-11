use std::{
    collections::HashSet,
    fmt, fs,
    path::{Path, PathBuf},
};

use derive_more::From;
use directories::ProjectDirs;
use iced::Point;
use log::warn;
use rust_decimal::Decimal;
use serde::{
    de::{self, Visitor},
    ser::SerializeTuple,
    Deserialize, Deserializer, Serialize, Serializer,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    /// In seconds, rounded to nearest tenth place
    pub elapsed: Option<Decimal>,
    pub last_pos: Option<Position>,
    pub on_top: Option<bool>,
    pub whitelist: Option<Whitelist>,
}

impl Config {
    pub fn load(location: impl AsRef<Path>) -> crate::Result<Self> {
        let serialized = fs::read_to_string(location)?;
        Ok(toml::from_str(&serialized)?)
    }

    pub fn load_or_fallback(location: Option<&PathBuf>) -> (Self, Option<PathBuf>) {
        location
            .and_then(|loc| Some((Self::load(loc).ok()?, None)))
            .unwrap_or_else(|| {
                let fallback = Self::get_default_project_file();
                let loaded = fallback.as_ref().and_then(|loc| Self::load(loc).ok());
                let data = loaded.unwrap_or_else(|| {
                    let assert_dir_failed = fallback
                        .as_ref()
                        .and_then(|file| file.parent())
                        .is_some_and(|parent| fs::create_dir_all(parent).is_err());
                    if assert_dir_failed {
                        warn!("Could not create default directory! Your time will not be saved.");
                    }
                    Self::default()
                });
                (data, fallback)
            })
    }

    pub fn save(&self, location: &PathBuf) -> crate::Result<()> {
        let serialized = toml::to_string(self)?;
        fs::write(location, serialized).map_err(Into::into)
    }

    pub fn get_default_project_file() -> Option<PathBuf> {
        Self::get_default_project_dir().map(|dir| dir.join("default.wtp"))
    }

    pub fn get_default_project_dir() -> Option<PathBuf> {
        ProjectDirs::from("dev", "reyma", "work-timer").map(|dirs| dirs.data_local_dir().to_owned())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            elapsed: None,
            on_top: Some(true),
            last_pos: None,
            whitelist: Some(Whitelist::default()),
        }
    }
}

#[derive(Clone, Debug, From)]
pub struct Position(Point);

impl From<Position> for Point {
    fn from(val: Position) -> Self {
        val.0
    }
}

impl Position {
    fn from(x: f32, y: f32) -> Self {
        Self(Point { x, y })
    }
}

impl Serialize for Position {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(&self.0.x)?;
        tup.serialize_element(&self.0.y)?;
        tup.end()
    }
}

impl<'de> Deserialize<'de> for Position {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple(2, PositionVisitor)
    }
}

struct PositionVisitor;

impl<'de> Visitor<'de> for PositionVisitor {
    type Value = Position;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "two f32 values")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        let Ok(Some(x)) = seq.next_element() else {
            return Err(de::Error::invalid_length(0, &self));
        };
        let Ok(Some(y)) = seq.next_element() else {
            return Err(de::Error::invalid_length(1, &self));
        };
        Ok(Position::from(x, y))
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Whitelist(HashSet<PathBuf>);

impl Whitelist {
    pub fn has(&self, needle: &PathBuf) -> bool {
        self.0.contains(needle)
    }

    pub fn toggle(&mut self, pathbuf: PathBuf) {
        if !self.0.remove(&pathbuf) {
            self.0.insert(pathbuf);
        }
    }

    pub fn set(&mut self, set: HashSet<PathBuf>) {
        self.0 = set;
    }
}

#[allow(clippy::implicit_hasher)]
impl From<Whitelist> for HashSet<PathBuf> {
    fn from(value: Whitelist) -> Self {
        value.0
    }
}
