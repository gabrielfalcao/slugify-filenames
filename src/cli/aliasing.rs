use heck::{
    ToKebabCase, ToLowerCamelCase, ToPascalCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase,
    ToTrainCase,
};
use std::fmt::Display;

pub fn heck_aliases<T: Display>(input: T) -> Vec<String> {
    let base = input.to_string();
    let mut aliases: Vec<String> = vec![base.to_string()];
    {
        let alias = base.to_lower_camel_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    {
        let alias = base.to_pascal_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    {
        let alias = base.to_train_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    {
        let alias = base.to_snake_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    {
        let alias = base.to_kebab_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    {
        let alias = base.to_shouty_snake_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    {
        let alias = base.to_shouty_kebab_case().to_string();
        if !aliases.contains(&alias) {
            aliases.push(alias);
        }
    }
    aliases
}
