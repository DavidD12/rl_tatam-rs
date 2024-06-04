use super::*;
use naming::*;

pub fn events_to_tatam(skillset: &Skillset) -> String {
    let mut out = "".to_string();

    out += "\n// ==================== Events ====================\n\n";

    for event in skillset.events() {
        out += &event_to_tatam(skillset, event);
    }

    out
}

pub fn event_to_tatam(skillset: &Skillset, event: &Event) -> String {
    let mut out = "".to_string();

    out += &format!("trans {} {{\n", event_name(skillset, event));
    out += &format!("\t{} = Free and ", skillset_var(skillset),);
    out += &format!(
        "{} and \n",
        expr_to_tatam(skillset, &(event.guard().clone().unwrap()))
    );
    //
    out += &format!(
        "\t|{}{}|(\n",
        skillset_var(skillset),
        effects_resources(event.effects())
            .iter()
            .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
            .fold("".to_string(), |acc, res| format!("{}, {}", acc, res))
    );

    out += &format!("\t\t{}' = Lock\n", skillset_var(skillset),);
    for effect in event.effects().iter() {
        let resource = skillset.get(effect.resource().resolved()).unwrap();
        let state = skillset.get(effect.state().resolved()).unwrap();
        out += &format!(
            "\t\tand {}' = {}\n",
            resource_var(skillset, resource),
            resource_state(skillset, state)
        );
    }

    out += "\t)\n";
    out += "}\n";

    out
}
