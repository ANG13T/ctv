use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub colors: Colors,
    pub field_order: Vec<FieldName>,
    pub max_depth: usize,
    pub show_metadata: ShowMetadataConfig,
    pub styles: FieldStyles,
    pub symbols: Symbols,
    pub time: TimeConfig,
    pub view_format: ViewFormat,
    pub sorting: Vec<SortMethod>,
}

#[derive(Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub struct SortMethod {
    #[serde(rename = "type")]
    pub ty: SortType,
    pub descending: bool,
}

impl<'de> Deserialize<'de> for SortMethod {
    fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        pub struct SortMethodProxy {
            #[serde(rename = "type")]
            ty: SortType,
            descending: bool,
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum WithOrWithoutOrder {
            WithoutOrder(SortType),
            WithOrder(SortMethodProxy),
        }
        let raw = WithOrWithoutOrder::deserialize(deserializer)?;
        Ok(match raw {
            WithOrWithoutOrder::WithOrder(SortMethodProxy { ty, descending }) => {
                SortMethod { ty, descending }
            }
            WithOrWithoutOrder::WithoutOrder(ty) => Self::from(ty),
        })
    }
}

impl From<SortType> for SortMethod {
    fn from(ty: SortType) -> Self {
        Self {
            ty,
            descending: false,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SortType {
    Type,
    Size,
    Name,
    Time,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FieldName {
    Owner,
    Perms,
    Size,
    Time,
}

pub struct FieldNameDisplay<'a> {
    field: FieldName,
    file: &'a crate::protocols::File<'a>,
}

impl FieldName {
    pub fn display<'a>(self, file: &'a crate::protocols::File<'a>) -> FieldNameDisplay<'a> {
        FieldNameDisplay { field: self, file }
    }
}

impl std::fmt::Display for FieldNameDisplay<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        use crate::services;
        match self.field {
            FieldName::Owner => {
                let owner: &str = &self.file.user;
                let owner = self.file.config.styles.owner.apply(owner);
                write!(formatter, "{}", owner)
            }
            FieldName::Perms => {
                let color = &self.file.config.colors.types;
                let letter = self.file.file_type.letter();
                let letter = self.file.file_type.color(color).apply(letter);
                write!(formatter, "{}{}", letter, self.file.perms)
            }
            FieldName::Size => {
                let size = services::size::format(self.file.size);
                let size = self.file.config.styles.size.apply(size.as_str());
                write!(formatter, "{}", size)
            }
            FieldName::Time => {
                let time = services::time::format(&self.file.time, &self.file.config.time.format);
                let time = self.file.config.styles.time.apply(time.as_str());
                write!(formatter, "{}", time)
            }
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            colors: Default::default(),
            field_order: vec![
                FieldName::Size,
                FieldName::Owner,
                FieldName::Perms,
                FieldName::Time,
            ],
            max_depth: 3,
            show_metadata: Default::default(),
            styles: Default::default(),
            symbols: Default::default(),
            time: Default::default(),
            view_format: Default::default(),
            sorting: vec![
                SortType::Type.into(),
                SortType::Name.into(),
                SortType::Size.into(),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShowMetadataConfig {
    pub directory: bool,
    pub file: bool,
}

impl Default for ShowMetadataConfig {
    fn default() -> Self {
        Self {
            directory: true,
            file: true,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeConfig {
    #[serde(rename = "type")]
    pub ty: TimeType,
    pub format: String,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            ty: Default::default(),
            format: "%Y-%m-%dT%H:%M:%S".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum TimeType {
    Created,
    Modified,
    Accessed,
}

impl Default for TimeType {
    fn default() -> Self {
        Self::Modified
    }
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Colors {
    pub types: TypeColors,
    pub perms: PermColors,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TypeColors {
    pub block_device: Color,
    pub char_device: Color,
    pub directory: Color,
    pub file: Color,
    pub pipe: Color,
    pub socket: Color,
    pub symlink: Color,
    pub unknown: Color,
}

impl Default for TypeColors {
    fn default() -> Self {
        Self {
            block_device: Color::BrightGreen,
            char_device: Color::BrightYellow,
            directory: Color::Blue,
            file: Color::None,
            pipe: Color::Yellow,
            socket: Color::BrightRed,
            symlink: Color::BrightPurple,
            unknown: Color::BrightBlack,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PermColors {
    pub execute: Color,
    pub none: Color,
    pub read: Color,
    pub write: Color,
}

impl Default for PermColors {
    fn default() -> Self {
        Self {
            execute: Color::BrightGreen,
            read: Color::BrightGreen,
            write: Color::BrightRed,
            none: Color::BrightBlack,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldStyles {
    pub owner: FieldStyle,
    pub size: FieldStyle,
    pub time: FieldStyle,
}

impl Default for FieldStyles {
    fn default() -> Self {
        Self {
            owner: FieldStyle {
                color: Color::BrightPurple,
                style: Style::None,
            },
            size: FieldStyle {
                color: Color::Blue,
                style: Style::Bold,
            },
            time: FieldStyle {
                color: Color::BrightCyan,
                style: Style::Bold,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FieldStyle {
    pub color: Color,
    pub style: Style,
}

impl FieldStyle {
    pub fn apply<C: colored::Colorize>(&self, base: C) -> colored::ColoredString {
        self.style.apply(self.color.apply(base))
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Color {
    None,
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightPurple,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn apply<C: Colorize>(self, base: C) -> colored::ColoredString {
        match self {
            Self::None => base.normal(),
            Self::Black => base.black(),
            Self::Red => base.red(),
            Self::Green => base.green(),
            Self::Yellow => base.yellow(),
            Self::Blue => base.blue(),
            Self::Purple => base.purple(),
            Self::Cyan => base.cyan(),
            Self::White => base.white(),
            Self::BrightBlack => base.bright_black(),
            Self::BrightRed => base.bright_red(),
            Self::BrightGreen => base.bright_green(),
            Self::BrightYellow => base.bright_yellow(),
            Self::BrightBlue => base.bright_blue(),
            Self::BrightPurple => base.bright_purple(),
            Self::BrightCyan => base.bright_cyan(),
            Self::BrightWhite => base.bright_white(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Style {
    None,
    Bold,
    Italic,
    Underline,
}

impl Style {
    pub fn apply(self, base: colored::ColoredString) -> colored::ColoredString {
        match self {
            // make sure to reapply the color if necessary
            Self::None => match base.fgcolor() {
                Some(color) => base.normal().color(color),
                None => base.normal(),
            },
            Self::Bold => base.bold(),
            Self::Italic => base.italic(),
            Self::Underline => base.underline(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ViewFormat {
    Short,
    Full,
}

impl Default for ViewFormat {
    fn default() -> Self {
        Self::Full
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Symbols {
    /// Used at the end of a sequence of entries
    pub elbow: String,
    /// Used at the beginning and middle of a sequence of entries
    pub tee: String,
    /// Added as a prefix to indent subdirectory trees
    pub pipe: String,
}

impl Default for Symbols {
    fn default() -> Self {
        Self {
            elbow: "└──".to_string(),
            tee: "├──".to_string(),
            pipe: "│".to_string(),
        }
    }
}

pub fn load() -> anyhow::Result<Config> {
    use figment::{
        providers::{Env, Format, Serialized, Toml},
        Figment,
    };
    let mut config = Figment::new();
    // load defaults
    config = config.merge(Serialized::from(
        Config::default(),
        figment::Profile::default(),
    ));
    // load from file if present
    if let Some(config_file_path) = config_file_path() {
        config = config.merge(Toml::file(config_file_path));
    }
    // load from environment
    config = config.merge(Env::prefixed("CTV_"));
    Ok(config.extract()?)
}

pub fn config_file_path() -> Option<PathBuf> {
    dirs::config_dir().map(|path| path.join("ctv.toml"))
}

#[cfg(test)]
mod test {
    #[test]
    fn test_sort_parsing() {
        use super::{SortMethod, SortType};
        use figment::providers::{Format, Toml};
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        struct Container {
            inner: SortMethod,
        }

        let as_string = r#"inner = "size""#;
        let as_object = r#"inner = { type = "time", descending = true }"#;

        let as_string: Container = Toml::from_str(as_string).unwrap();
        let as_object: Container = Toml::from_str(as_object).unwrap();

        assert_eq!(
            as_string.inner,
            SortMethod {
                ty: SortType::Size,
                descending: false
            }
        );
        assert_eq!(
            as_object.inner,
            SortMethod {
                ty: SortType::Time,
                descending: true
            }
        );
    }
}
