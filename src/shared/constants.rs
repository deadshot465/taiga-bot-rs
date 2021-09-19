use once_cell::sync::Lazy;
use regex::Regex;
use serenity::utils::Color;

pub const ASSET_DIRECTORY: &str = "assets";
pub const CONFIG_DIRECTORY: &str = "config";
pub const RECORD_DIRECTORY: &str = "records";
pub const KOU_COLOR: Color = Color::new(0xe7a43a);
pub const TAIGA_COLOR: Color = Color::new(0xe81615);
pub const RUST_LOGO: &str = "https://cdn.discordapp.com/emojis/448579316171669545.png";
pub const CAMP_BUDDY_STAR: &str = "https://cdn.discordapp.com/emojis/593518771554091011.png";

pub static EMOTE_IS_ANIMATED_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(<a)").expect("Failed to initialize regular expression."));
