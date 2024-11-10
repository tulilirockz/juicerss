use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FeedConfigEntry {
    pub name: Option<String>,
    pub url: String,
    #[serde(default)]
    pub enabled: bool,
    pub filter: Option<String>,
}

impl Default for FeedConfigEntry {
    fn default() -> Self {
        Self {
            name: None,
            url: String::new(),
            enabled: true,
            filter: None,
        }
    }
}

// Purely just a workaround since it is very annoying to parse stuff from #(whatever)
// TODO: Parse string properly without it being like this
#[derive(Debug, Default, Deserialize, Clone)]
pub struct ColorConfiguration {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Deserialize, Clone)]
// RGB values
pub struct ThemeConfiguration {
    #[serde(default)]
    pub accent: ColorConfiguration,
    #[serde(default)]
    pub text: ColorConfiguration,
    #[serde(default)]
    pub error: ColorConfiguration,
}

#[derive(Debug, Default, Deserialize, Clone)]
pub enum ListFormat {
    #[default]
    #[serde(alias = "compact", alias = "default")]
    Compact,
    #[serde(alias = "extended")]
    Extended,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RendererConfiguration {
    pub enabled: Option<bool>,
    pub binary: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ScrollingConfiguration {
    pub x_factor: u16,
    pub x_lines: u16,
    pub y_factor: u16,
    pub y_lines: u16,
}

impl Default for ScrollingConfiguration {
    fn default() -> Self {
        Self {
            x_factor: 1,
            x_lines: 1,
            y_factor: 1,
            y_lines: 1,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default)]
    pub nerd_fonts: bool,
    #[serde(default)]
    pub list_format: ListFormat,
    #[serde(default)]
    pub feeds: Option<Vec<FeedConfigEntry>>,
    #[serde(default)]
    pub theme: ThemeConfiguration,
    #[serde(default)]
    pub renderer: Option<RendererConfiguration>,
    #[serde(default)]
    pub scrolling: ScrollingConfiguration,
}

impl Default for ThemeConfiguration {
    fn default() -> Self {
        Self {
            error: ColorConfiguration {
                red: 255,
                green: 0,
                blue: 0,
            },
            accent: ColorConfiguration {
                red: 83,
                green: 117,
                blue: 252,
            },
            text: ColorConfiguration {
                red: 0xFF,
                green: 0xFF,
                blue: 0xFF,
            },
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            feeds: None,
            nerd_fonts: true,
            list_format: ListFormat::Compact,
            theme: ThemeConfiguration::default(),
            renderer: None,
            scrolling: ScrollingConfiguration::default(),
        }
    }
}
