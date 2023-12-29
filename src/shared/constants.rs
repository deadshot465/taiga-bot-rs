use once_cell::sync::Lazy;
use regex::Regex;
use serenity::all::Color;

pub const ASSET_DIRECTORY: &str = "assets";
pub const CONFIG_DIRECTORY: &str = "config";
pub const RECORD_DIRECTORY: &str = "records";
pub const KOU_COLOR: Color = Color::new(0xe7a43a);
pub const TAIGA_COLOR: Color = Color::new(0xe81615);
pub const RUST_LOGO: &str = "https://cdn.discordapp.com/emojis/448579316171669545.png";
pub const CAMP_BUDDY_STAR: &str = "https://cdn.discordapp.com/emojis/593518771554091011.png";

pub const KOU_SERVER_ID: u64 = 705036924330704968;
pub const KOU_SERVER_ADMIN_ROLE_ID: u64 = 706778860812894228;
pub const KOU_SERVER_RULE_CHANNEL_ID: u64 = 722824790972563547;
pub const KOU_SERVER_CERTIFIED_ROLE_ID: u64 = 736534226945572884;
pub const KOU_SERVER_SMOTE_ROLE_ID: u64 = 771070164363903028;
pub const KOU_SERVER_CERTIFICATION_MESSAGE: &str = "I agree with the rule and Kou is the best boi.";
pub const KOU_SERVER_QOTD_CHANNEL_ID: u64 = 727519983986278411;

pub const TAIGA_SERVER_ID: u64 = 696414250406510623;
pub const TAIGA_SERVER_ADMIN_ROLE_ID: u64 = 742061690824294520;
pub const TAIGA_SERVER_WINTER_SPLENDOR_ROLE_ID: u64 = 697879312988241981;
pub const TAIGA_SERVER_SMOTE_ROLE_ID: u64 = 766023350287335465;

pub const SHIBA_KEK_ICON: &str = "https://cdn.discordapp.com/emojis/730239295155077251.png";

pub const EMOTE_BASE_LINK: &str = "https://cdn.discordapp.com/emojis/";

pub static EMOTE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(<a?:\w+:\d+>)").expect("Failed to initialize regular expression."));

pub static EMOTE_ID_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(:\w+:)(\d+)").expect("Failed to initialize regular expression."));

pub static EMOTE_IS_ANIMATED_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(<a)").expect("Failed to initialize regular expression."));
