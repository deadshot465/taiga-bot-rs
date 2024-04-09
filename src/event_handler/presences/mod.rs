use crate::shared::structs::ContextData;
use rand::prelude::*;
use serenity::all::ActivityData;
use serenity::futures::prelude::future::BoxFuture;
use serenity::model::prelude::OnlineStatus;
use serenity::prelude::*;
use serenity::FutureExt;

pub async fn set_initial_presence(ctx: &Context, data: &ContextData) {
    set_activity(ctx, &data.common_settings.activities).await;
    let ctx_clone = ctx.clone();
    let activities = data.common_settings.activities.clone();
    tokio::spawn(async move { update_presence(ctx_clone, activities).await });
}

async fn set_activity(ctx: &Context, activities: &[String]) {
    let activity = {
        let mut rng = thread_rng();
        activities.choose(&mut rng)
    };

    if let Some(activity) = activity {
        let activity = ActivityData::playing(activity);
        ctx.set_presence(Some(activity), OnlineStatus::Online);
    }
}

fn update_presence(ctx: Context, activities: Vec<String>) -> BoxFuture<'static, ()> {
    async move {
        tokio::time::sleep(std::time::Duration::from_secs(3600)).await;
        set_activity(&ctx, &activities).await;
        tokio::spawn(async move { update_presence(ctx, activities).await });
    }
    .boxed()
}
