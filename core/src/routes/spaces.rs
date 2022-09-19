use rand::seq::SliceRandom;
use rspc::RouterBuilder;

use crate::Shared;

pub fn mount() -> RouterBuilder<Shared> {
    RouterBuilder::<Shared>::new()
        .mutation("create", move |ctx, _: ()| async move {
            let client = ctx.client.clone();
            let space_len = client.spaces().count(vec![]).exec().await?;
            let colors = vec![
                "#F16A50", "#CA3214", "#FF6369", "#D864D8", "#849DFF", "#3451B2", "#00C2D7",
            ];
            let color = colors
                .choose(&mut rand::thread_rng())
                .unwrap()
                .to_owned()
                .to_owned();

            let space = client
                .spaces()
                .create(
                    "Spaces".to_owned() + &space_len.to_string(),
                    "Document16Filled".to_owned(),
                    color,
                    vec![],
                )
                .exec()
                .await?;

            Ok(space)
        })
        .query("get", move |ctx, _: ()| async move {
            let client = ctx.client.clone();
            let spaces = client.spaces().find_many(vec![]).exec().await?;

            Ok(spaces)
        })
}