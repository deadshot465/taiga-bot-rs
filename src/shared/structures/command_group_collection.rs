use serenity::prelude::TypeMapKey;
use serenity::framework::standard::CommandGroup;

pub struct CommandGroupCollection;
impl TypeMapKey for CommandGroupCollection {
    type Value = Vec<&'static CommandGroup>;
}