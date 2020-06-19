use regex::Regex;
use serenity::model::guild::{Guild, Member};
use serenity::model::id::UserId;
use serenity::prelude::Context;

pub async fn search_user(context: &Context, guild: &Guild, query: &str) -> Result<Vec<Member>, Box<dyn std::error::Error>> {

    lazy_static! {
        static ref USER_MENTION_REGEX: Regex = Regex::new(r"<@!?(\d{17,20})>").unwrap();
        static ref USER_TAG: Regex = Regex::new(r"(\S.{0,30}\S)\s*#(\d{4})").unwrap();
        static ref DISCORD_ID: Regex = Regex::new(r"\d{17,20}").unwrap();
    }

    if query.len() <= 0 {
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
    }
    else if USER_TAG.is_match(query) {
        let captures = USER_TAG.captures(query).unwrap();
        let user_name = captures.get(1).unwrap().as_str();
        let user_discriminator = captures.get(2).unwrap().as_str().parse::<u16>()?;
        let ref members = guild.members;
        let result = members
            .iter()
            .find(|x| (((*x).1.nick.is_some() && (*x).1.nick.as_ref().unwrap() == user_name) || (*x).1.user.name == user_name) && (*x).1.user.discriminator == user_discriminator);
        if let Some(x) = result {
            member = Some(x.1.clone());
        }
    }
    else if DISCORD_ID.is_match(query) {
        let id = DISCORD_ID.captures(query).unwrap().get(1).unwrap().as_str().parse::<u64>()?;
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
        if (user.nick.is_some() && user.nick.as_ref().unwrap() == query) || user.user.name == query {
            exact_match.push(user.clone());
        }
        else if (ignore_case(user.user.name.as_str(), query) ||
            (user.nick.is_some() && ignore_case(user.nick.as_ref().unwrap().as_str(), query))) &&
            exact_match.len() <= 0 {
            wrong_case.push(user.clone());
        }
        else if (user.user.name.to_lowercase().starts_with(query) ||
            (user.nick.is_some() && user.nick.as_ref().unwrap().to_lowercase().starts_with(query))) &&
            wrong_case.len() <= 0 {
            starts_with.push(user.clone());
        }
        else if (user.user.name.to_lowercase().contains(lower_query.as_str()) ||
            (user.nick.is_some() && user.nick.as_ref().unwrap().to_lowercase().contains(lower_query.as_str()))) &&
            starts_with.len() <= 0 {
            contains.push(user.clone());
        }
    }

    exact_match.append(&mut wrong_case);
    exact_match.append(&mut starts_with);
    exact_match.append(&mut contains);
    return Ok(exact_match);
}

fn ignore_case<'a>(str1: &'a str, str2: &'a str) -> bool {
    if str1.len() == 0 || str2.len() == 0 || str1.is_empty() || str2.is_empty() {
        return false;
    }
    str1.to_uppercase() == str2.to_uppercase()
}