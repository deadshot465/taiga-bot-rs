use serenity::model::guild::Member;
use serenity::model::prelude::User;

pub fn get_animated_emote_url(emote_id: &str) -> String {
    format!("https://cdn.discordapp.com/emojis/{}.gif?v=1", emote_id)
}

pub fn get_author_avatar(user: &User) -> String {
    user.avatar_url().unwrap_or(user.default_avatar_url())
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

/*pub async fn search_user(
    context: &Context,
    guild: &Guild,
    query: &str,
) -> Result<Vec<Member>, CommandError> {
    lazy_static! {
        static ref USER_MENTION_REGEX: Regex = Regex::new(r"<@!?(\d{17,20})>").unwrap();
        static ref USER_TAG: Regex = Regex::new(r"(\S.{0,30}\S)\s*#(\d{4})").unwrap();
        static ref DISCORD_ID: Regex = Regex::new(r"\d{17,20}").unwrap();
    }

    if query.is_empty() {
        return Ok(vec![]);
    }

    let mut member: Option<Member> = None;
    if USER_MENTION_REGEX.is_match(query) {
        let id = USER_MENTION_REGEX
            .captures(query)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()?;
        let user_id = UserId::from(id);
        let result = guild.member(&context.http, user_id).await;
        if let Ok(x) = result {
            member = Some(x);
        }
    } else if USER_TAG.is_match(query) {
        let captures = USER_TAG.captures(query).unwrap();
        let user_name = captures.get(1).unwrap().as_str();
        let user_discriminator = captures.get(2).unwrap().as_str().parse::<u16>()?;
        let members = &guild.members;
        let result = members.iter().find(|x| {
            (((*x).1.nick.is_some() && (*x).1.nick.as_ref().unwrap() == user_name)
                || (*x).1.user.name == user_name)
                && (*x).1.user.discriminator == user_discriminator
        });
        if let Some(x) = result {
            member = Some(x.1.clone());
        }
    } else if DISCORD_ID.is_match(query) {
        let id = DISCORD_ID
            .captures(query)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .parse::<u64>()?;
        let user_id = UserId::from(id);
        let result = guild.member(&context.http, user_id).await;
        if let Ok(x) = result {
            member = Some(x);
        }
    }

    if let Some(x) = member {
        return Ok(vec![x]);
    }

    let mut exact_match: Vec<Member> = vec![];
    let mut wrong_case: Vec<Member> = vec![];
    let mut starts_with: Vec<Member> = vec![];
    let mut contains: Vec<Member> = vec![];
    let lower_query = query.to_lowercase();

    for (_, user) in guild.members.iter() {
        if (user.nick.is_some() && user.nick.as_ref().unwrap() == query) || user.user.name == query
        {
            exact_match.push(user.clone());
        } else if (ignore_case(user.user.name.as_str(), query)
            || (user.nick.is_some() && ignore_case(user.nick.as_ref().unwrap().as_str(), query)))
            && exact_match.is_empty()
        {
            wrong_case.push(user.clone());
        } else if (user.user.name.to_lowercase().starts_with(query)
            || (user.nick.is_some()
                && user
                    .nick
                    .as_ref()
                    .unwrap()
                    .to_lowercase()
                    .starts_with(query)))
            && wrong_case.is_empty()
        {
            starts_with.push(user.clone());
        } else if (user.user.name.to_lowercase().contains(lower_query.as_str())
            || (user.nick.is_some()
                && user
                    .nick
                    .as_ref()
                    .unwrap()
                    .to_lowercase()
                    .contains(lower_query.as_str())))
            && starts_with.is_empty()
        {
            contains.push(user.clone());
        }
    }

    exact_match.append(&mut wrong_case);
    exact_match.append(&mut starts_with);
    exact_match.append(&mut contains);
    Ok(exact_match)
}

fn ignore_case<'a>(str1: &'a str, str2: &'a str) -> bool {
    if str1.is_empty() || str2.is_empty() {
        return false;
    }
    str1.to_uppercase() == str2.to_uppercase()
}
*/
