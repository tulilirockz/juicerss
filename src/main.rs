use clap::Parser;
use feed_rs::model::{Entry, Feed};
mod tui;
use regex::Regex;
use serde::Deserialize;
use tokio::task::JoinSet;
use tui::App;

#[derive(Debug, Deserialize, Clone)]
struct FeedConfigEntry {
    name: Option<String>,
    url: String,
    #[serde(default)]
    enabled: bool,
    filter: Option<String>,
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
struct ColorConfiguration {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug, Deserialize, Clone)]
// RGB values
struct ThemeConfiguration {
    #[serde(default)]
    accent: ColorConfiguration,
    #[serde(default)]
    text: ColorConfiguration,
    #[serde(default)]
    error: ColorConfiguration,
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
pub struct Config {
    #[serde(default)]
    nerd_fonts: bool,
    #[serde(default)]
    list_format: ListFormat,
    #[serde(default)]
    feeds: Option<Vec<FeedConfigEntry>>,
    #[serde(default)]
    theme: ThemeConfiguration,
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct FeedWithCustom {
    feed: Feed,
    filtered_entries: Vec<Entry>,
    name: Option<String>,
}

#[derive(Parser)]
struct Cli {
    #[clap(help = "Feeds to be read")]
    feeds: Vec<String>,

    #[clap(short, long, help = "Path for configuration file that will be used")]
    config: Option<String>,
}

#[derive(Default)]
struct FeedResponse {
    body: String,
    url: String,
    filter: Option<String>,
    name: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Cli::parse();

    let app_config: Config = args.config.map_or_else(Config::default, |config| {
        let config_content =
            std::fs::read_to_string(config).expect("Failed reading configuration file");
        toml::from_str(&config_content).expect("Failed parsing configuration file")
    });

    let config_feeds: Option<Vec<FeedConfigEntry>> = if args.feeds.is_empty() {
        app_config
            .feeds
            .clone()
            .map(|feeds| feeds.into_iter().filter(|e| e.enabled).collect())
    } else {
        Some(
            args.feeds
                .iter()
                .map(|e| FeedConfigEntry {
                    url: e.to_string(),
                    ..Default::default()
                })
                .collect(),
        )
    };

    let mut set = JoinSet::new();

    config_feeds
        .expect("No feeds were specified")
        .into_iter()
        .map(|e| async move {
            FeedResponse {
                body: reqwest::get(&e.url).await.unwrap().text().await.unwrap(),
                url: e.url,
                name: e.name,
                filter: e.filter,
            }
        })
        .for_each(|e| {
            set.spawn(e);
        });

    let feeds: Vec<_> = set
        .join_all()
        .await
        .iter()
        .map(|response| {
            let parsed = feed_rs::parser::parse(response.body.as_bytes()).ok();

            let parsed_feed = parsed?;

            let filtered_entries: Vec<Entry> = parsed_feed
                .clone()
                .entries
                .into_iter()
                .filter(|entry| {
                    let Some(pattern) = &response.filter else {
                        return true;
                    };

                    let Some(title) = &entry.title else {
                        return false;
                    };

                    Regex::new(pattern.clone().as_str())
                        .unwrap_or_else(|_| panic!("Invalid regex specified in {}", response.url))
                        .is_match(&title.content)
                })
                .collect();

            Some(FeedWithCustom {
                feed: parsed_feed,
                filtered_entries,
                name: response.name.clone(),
            })
        })
        .collect();

    let terminal = ratatui::init();
    App::new(feeds, app_config).run(terminal);
    ratatui::restore();
    Ok(())
}
