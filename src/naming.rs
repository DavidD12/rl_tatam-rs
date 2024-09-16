use super::*;
use heck::{ToSnakeCase, ToUpperCamelCase};

pub fn string_string(first_string: &str, second_string: &str) -> String {
    format!("{}_{}", first_string, second_string)
}

// Skillset

pub fn skillset_var(skillset: &Skillset) -> String {
    format!("{}_state", skillset.name().to_snake_case())
}

pub fn skillset_fact_pred_name(skillset: &Skillset) -> String {
    skillset.name().to_snake_case()
}

pub fn success_pred_name(success: &Success) -> String {
    format!("success_{}", success.name().to_snake_case())
}

pub fn failure_pred_name(failure: &Failure) -> String {
    format!("failure_{}", failure.name().to_snake_case())
}

// Skill

pub fn skill_fact_pred_name(skillset: &Skillset, skill: &Skill) -> String {
    string_string(
        &skillset.name().to_snake_case(),
        &skill.name().to_snake_case(),
    )
}

pub fn skill_var(skillset: &Skillset, skill: &Skill) -> String {
    string_string(
        &skillset.name().to_snake_case(),
        &skill.name().to_snake_case(),
    ) + "_state"
}

// Interface

pub fn interface_var(skillset: &Skillset, skill: &Skill, composite_name: &String) -> String {
    string_string(&composite_name.to_snake_case(), &string_string(
        &skillset.name().to_snake_case(),
        &skill.name().to_snake_case(),
    )) + "_interface_state"
}

pub fn interface_result_var(skillset: &Skillset, skill: &Skill, composite_name: &String) -> String {
    string_string(&composite_name.to_snake_case(), &string_string(
        &skillset.name().to_snake_case(),
        &skill.name().to_snake_case(),
    )) + "_result"
}

// Resource

pub fn resource_enum(skillset: &Skillset, resource: &Resource) -> String {
    string_string(
        &skillset.name().to_upper_camel_case(),
        &resource.name().to_upper_camel_case(),
    )
}

pub fn resource_state(skillset: &Skillset, state: &State) -> String {
    string_string(
        &skillset.name().to_upper_camel_case(),
        &state.name().to_upper_camel_case(),
    )
}

pub fn resource_var(skillset: &Skillset, resource: &Resource) -> String {
    string_string(
        &skillset.name().to_snake_case(),
        &resource.name().to_snake_case(),
    )
}

// Event

pub fn event_name(skillset: &Skillset, event: &Event) -> String {
    string_string(
        &skillset.name().to_snake_case(),
        &event.name().to_snake_case(),
    )
}
