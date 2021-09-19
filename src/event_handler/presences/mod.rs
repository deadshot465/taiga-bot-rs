use crate::shared::structs::config::common_settings::COMMON_SETTINGS;
use rand::prelude::*;
use serenity::futures::prelude::future::BoxFuture;
use serenity::model::prelude::{Activity, OnlineStatus};
use serenity::prelude::*;
use serenity::FutureExt;

pub async fn set_initial_presence(ctx: &Context) {
    set_activity(ctx).await;
    let ctx_clone = ctx.clone();
    tokio::spawn(async move { update_presence(ctx_clone).await });
}

async fn set_activity(ctx: &Context) {
    let activity = {
        let mut rng = rand::thread_rng();
        COMMON_SETTINGS.activities.choose(&mut rng)
    };

    if let Some(activity) = activity {
        let activity = Activity::playing(activity);
        ctx.set_presence(Some(activity), OnlineStatus::Online).await;
    }
}

fn update_presence(ctx: Context) -> BoxFuture<'static, ()> {
    async move {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        set_activity(&ctx).await;
        tokio::spawn(async move { update_presence(ctx).await });
    }
    .boxed()
}
