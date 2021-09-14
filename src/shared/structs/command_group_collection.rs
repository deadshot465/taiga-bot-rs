use serenity::framework::standard::CommandGroup;
use serenity::prelude::TypeMapKey;

pub struct CommandGroupCollection;
impl TypeMapKey for CommandGroupCollection {
    type Value = Vec<&'static CommandGroup>;
}
