use serenity::all::{Message, UserId};
use serenity::model::guild::Member;
use serenity::model::prelude::User;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn find_user_in_members<'a>(user: &'a User, members: &'a [Member]) -> Option<&'a Member> {
    members
        .iter()
        .find(|member| member.user.id.get() == user.id.get())
}

pub fn get_animated_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{emote_id}.gif?v=1")
}

pub fn get_author_avatar(user: &User) -> String {
    user.avatar_url()
        .unwrap_or_else(|| user.default_avatar_url())
}

pub fn get_author_name(user: &User, member: &Option<Member>) -> String {
    if let Some(m) = member {
        m.display_name().to_string()
    } else {
        user.name.clone()
    }
}

pub fn get_first_name(name: &str) -> &str {
    let first_name: Vec<&str> = name.split(' ').collect();
    first_name[0]
}

pub fn get_static_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{emote_id}.png?v=1")
}

pub fn build_author_name_map(messages: &[Message]) -> HashMap<UserId, String> {
    let mut author_name_map = HashMap::new();

    for message in messages {
        let user_id = message.author.id;
        if let Entry::Vacant(e) = author_name_map.entry(user_id) {
            let name = message.author.name.clone();
            e.insert(name);
        }
    }

    author_name_map
}
