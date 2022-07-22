use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::guild::Member;
use serenity::model::prelude::User;

pub fn extract_string_option(command: &ApplicationCommandInteraction, index: usize) -> &str {
    command
        .data
        .options
        .get(index)
        .and_then(|opt| opt.value.as_ref())
        .and_then(|value| value.as_str())
        .unwrap_or_default()
}

pub fn find_user_in_members(user: User, members: &[Member]) -> Option<&Member> {
    members.iter().find(|member| member.user.id.0 == user.id.0)
}

pub fn get_animated_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.gif?v=1", emote_id)
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
    format!("https://cdn.discordapp.com/emojis/{}.png?v=1", emote_id)
}
