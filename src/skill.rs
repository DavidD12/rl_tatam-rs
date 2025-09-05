use super::*;

pub fn skill_to_tatam(skillset: &Skillset, skill: &Skill, composite_skill_names: &Vec<String>, label: bool, interrupt: bool) -> ModelTransNames {
    let mut out = String::new();
    let mut trans_names : Vec<String> = vec![];

    out += &format!(
        "\n// -------------------- Skill {} --------------------\n",
        skill.name()
    );

    out += &format!("\nvar {}: SkillState\n", skill_var(skillset, skill));

    out += &format!("\ninit {}_{} {{\n", skillset.name(), skill.name());
    out += &format!("\t{} = Idle\n", skill_var(skillset, skill));
    out += "}\n";
    
    if composite_skill_names.is_empty() {
        let result = skill_idle_to_running(skillset, skill, &String::new(), label);
        out += &result.model;
        trans_names.extend(result.transition_names);
    } else {
        for compo_name in composite_skill_names {
            out += &skill_idle_to_idle_precond_false(skillset, skill, compo_name);
            let result = skill_idle_to_running(skillset, skill, compo_name, label);
            out += &result.model;
            trans_names.extend(result.transition_names);
            out += &skill_running_to_running_busy(skillset, skill, compo_name);
        }
    }

    for success in skill.successes() {
        let result = skill_running_to_success(skillset, skill, success, label);
        out += &result.model;
        trans_names.extend(result.transition_names);
    }

    for failure in skill.failures() {
        let result = skill_running_to_failure(skillset, skill, failure, label);
        out += &result.model;
        trans_names.extend(result.transition_names);
    }

    if interrupt {
        let result = skill_running_to_interrupting(skillset, skill, label);
        out += &result.model;
        trans_names.extend(result.transition_names);
        let result = skill_interrupting_to_interrupted(skillset, skill, label);
        out += &result.model;
        trans_names.extend(result.transition_names);
    }

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}

fn skill_idle_to_idle_precond_false(skillset: &Skillset, skill: &Skill, compo_name: &String) -> String {
    let mut out = String::new();

    out += &format!(
        "trans {}_{}_{}_idle_to_idle_precond_false {{\n",
        &compo_name,
        skillset.name(),
        skill.name()
    );
    out += &format!(
        "\t{} = Free and {} = Idle\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    out += "\tand not ( ";
    for precond in skill.preconditions().iter() {
        out += &format!("{} and", expr_to_tatam(skillset, precond.expr()));
    }
    out.truncate(out.len()-3);
    out += ")\n";
    out += &format!("\tand {} = AsynCall\n", &interface_var(skillset, skill, &compo_name));
    out += &format!("\tand |{}, {}|(\n", &interface_var(skillset, skill, &compo_name), &interface_result_var(skillset, skill, &compo_name));
    
    out += &format!(
        "\t\t{}' = NoCall and {}' = StartFailureRes\n",
        &interface_var(skillset, skill, &compo_name),
        &interface_result_var(skillset, skill, &compo_name)
    );

    out += "\t)\n";
    out += "}\n";

    out
}

fn skill_idle_to_running(skillset: &Skillset, skill: &Skill, compo_name: &String, label: bool) -> ModelTransNames {
    let mut out = String::new();
    let mut trans_names = vec![];

    out += "trans ";
    if !compo_name.is_empty() {
        out += &format!("{}_",&compo_name);
    }
    out += &format!(
        "{}_{}_idle_to_running {{\n",
        skillset.name(),
        skill.name()
    );
    trans_names.push(format!("{}_{}_idle_to_running", skillset.name(), skill.name()));
    out += &format!(
        "\t{} = Free and {} = Idle\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    for precond in skill.preconditions().iter() {
        out += &format!("\tand {}\n", expr_to_tatam(skillset, precond.expr()));
    }
    if !compo_name.is_empty() {
        out += &format!("\tand {} = AsynCall\n", &interface_var(skillset, skill, &compo_name));
    }
        // Used Resources
    let mut resources: HashSet<ResourceId> = HashSet::default();
    resources.extend(effects_resources(skill.start()));
    out += &format!(
        "\tand |{}, {}",
        skillset_var(skillset),
        resources
            .iter()
            .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
            .fold(skill_var(skillset, skill), |acc, res| format!(
                "{}, {}",
                acc, res
            ))
    );
    if !compo_name.is_empty() {
        out += &format!(", {}", &interface_var(skillset, skill, &compo_name));
    }
    if label {
        out += &format!(", {}", &trans_label_variable());
    }
    out += "|(\n";
    out += &format!(
        "\t\t{}' = Lock and {}' = Running\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Effects
    for effect in skill.start() {
        out += &format!(
            "\t\tand {}' = {}\n",
            resource_var(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap()
            ),
            resource_state(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap(),
                skillset.get(effect.state().resolved()).unwrap()
            )
        );
    }
    if !compo_name.is_empty() {
        out += &format!("\t\tand {}' = WaitResult\n", &interface_var(skillset, skill, &compo_name));
    }

    if label {
        out += &format!("\t\t\tand {}' = {}\n", &trans_label_variable(), &trans_label_enum_value(&format!("{}_{}_idle_to_running", skillset.name(), skill.name())));
    }

    out += "\t)\n";
    out += "}\n";

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}

fn skill_running_to_running_busy(skillset: &Skillset, skill: &Skill, compo_name: &String) -> String {
    let mut out = String::new();

    out += &format!(
        "trans {}_{}_{}_running_to_running_busy {{\n",
        &compo_name,
        skillset.name(),
        skill.name()
    );
    out += &format!(
        "\t{} = Free and {} = Running\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    out += &format!("\tand {} = AsynCall\n", &interface_var(skillset, skill, &compo_name));
    out += &format!("\tand |{}, {}|(\n", &interface_var(skillset, skill, &compo_name), &interface_result_var(skillset, skill, &compo_name));
    
    out += &format!(
        "\t\t{}' = NoCall and {}' = SkillBusyRes\n",
        &interface_var(skillset, skill, &compo_name),
        &interface_result_var(skillset, skill, &compo_name)
    );

    out += "\t)\n";
    out += "}\n";

    out
}

fn skill_running_to_success(skillset: &Skillset, skill: &Skill, success: &Success, label: bool) -> ModelTransNames {
    let mut out = String::new();
    let mut trans_names = vec![];

    out += &format!(
        "trans {}_{}_running_to_success_{} {{\n",
        skillset.name(),
        skill.name(),
        success.name()
    );
    trans_names.push(format!( "{}_{}_running_to_success_{}", skillset.name(), skill.name(), success.name()));
    out += &format!(
        "\t{} = Free and {} = Running\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Used Resources
    let mut resources: HashSet<ResourceId> = HashSet::default();
    resources.extend(effects_resources(success.effects()));
    resources.extend(postconsitions_resources(success.postconditions()));

    out += &format!(
        "\tand |{}, {}",
        skillset_var(skillset),
        resources
            .iter()
            .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
            .fold(skill_var(skillset, skill), |acc, res| format!(
                "{}, {}",
                acc, res
            ))
    );
    if label {
        out += &format!(", {}", &trans_label_variable())
    }
    out += "|(\n";

    out += &format!(
        "\t\t{}' = Lock and {}' = Success\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Effects
    for effect in success.effects() {
        out += &format!(
            "\t\tand {}' = {}\n",
            resource_var(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap()
            ),
            resource_state(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap(),
                skillset.get(effect.state().resolved()).unwrap()
            )
        );
    }
    // Postconditions
    for post in success.postconditions() {
        let expr = post.expr();
        out += &format!("\t\tand {}\n", next_expr_to_tatam(skillset, expr));
    }
    if label {
        out += &format!("\t\t\tand {}' = {}\n", &trans_label_variable(), &trans_label_enum_value(&format!( "{}_{}_running_to_success_{}", skillset.name(), skill.name(), success.name())));
    }

    out += "\t)\n";
    out += "}\n";

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}

fn skill_running_to_failure(skillset: &Skillset, skill: &Skill, failure: &Failure, label: bool) -> ModelTransNames {
    let mut out = String::new();
    let mut trans_names = vec![];

    out += &format!(
        "trans {}_{}_running_to_failure_{} {{\n",
        skillset.name(),
        skill.name(),
        failure.name()
    );
    trans_names.push(format!( "{}_{}_running_to_failure_{}", skillset.name(), skill.name(), failure.name()));
    out += &format!(
        "\t{} = Free and {} = Running\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Used Resources
    let mut resources: HashSet<ResourceId> = HashSet::default();
    resources.extend(effects_resources(failure.effects()));
    resources.extend(postconsitions_resources(failure.postconditions()));

    out += &format!(
        "\tand |{}, {}",
        skillset_var(skillset),
        resources
            .iter()
            .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
            .fold(skill_var(skillset, skill), |acc, res| format!(
                "{}, {}",
                acc, res
            ))
    );
    if label {
        out += &format!(", {}", &trans_label_variable())
    }
    out += "|(\n";
    
    out += &format!(
        "\t\t{}' = Lock and {}' = Failure\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Effects
    for effect in failure.effects() {
        out += &format!(
            "\t\tand {}' = {}\n",
            resource_var(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap()
            ),
            resource_state(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap(),
                skillset.get(effect.state().resolved()).unwrap()
            )
        );
    }
    // Postconditions
    for post in failure.postconditions() {
        let expr = post.expr();
        out += &format!("\t\tand {}\n", next_expr_to_tatam(skillset, expr));
    }

    if label {
        out += &format!("\t\t\tand {}' = {}\n", &trans_label_variable(), &trans_label_enum_value(&format!( "{}_{}_running_to_failure_{}", skillset.name(), skill.name(), failure.name())));
    }

    out += "\t)\n";
    out += "}\n";

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}

fn skill_running_to_interrupting(skillset: &Skillset, skill: &Skill, label: bool) -> ModelTransNames {
    let mut out = String::new();
    let mut trans_names = vec![];

    out += &format!(
        "trans {}_{}_running_to_interrupting {{\n",
        skillset.name(),
        skill.name()
    );
    trans_names.push(format!( "{}_{}_running_to_interrupting", skillset.name(), skill.name()));
    out += &format!(
        "\t{} = Free and {} = Running\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Effect
    if label {
        out += &format!(
            "\tand |{}, {}|({}' = Interrupting and {}' = {})\n",
            skill_var(skillset, skill),
            &trans_label_variable(),
            skill_var(skillset, skill),
            &trans_label_variable(),
            &trans_label_enum_value(&format!( "{}_{}_running_to_interrupting", skillset.name(), skill.name()))
        );
    } else {
        out += &format!(
            "\tand |{}|({}' = Interrupting)\n",
            skill_var(skillset, skill),
            skill_var(skillset, skill)
        );
    }
    
    out += "}\n";

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}

fn skill_interrupting_to_interrupted(skillset: &Skillset, skill: &Skill, label: bool) -> ModelTransNames {
    let mut out = String::new();
    let mut trans_names = vec![];

    out += &format!(
        "trans {}_{}_interrupting_to_interrupted {{\n",
        skillset.name(),
        skill.name(),
    );
    trans_names.push(format!( "{}_{}_interrupting_to_interrupted", skillset.name(), skill.name()));
    out += &format!(
        "\t{} = Free and {} = Interrupting\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Effects & Postconditions
    let effects = if let Some(interrupt) = skill.interrupt() {
        interrupt.effects().clone()
    } else {
        vec![]
    };
    let postconditions = if let Some(interrupt) = skill.interrupt() {
        interrupt.postconditions().clone()
    } else {
        vec![]
    };
    // Used Resources
    let mut resources: HashSet<ResourceId> = HashSet::default();
    resources.extend(effects_resources(&effects));
    resources.extend(postconsitions_resources(&postconditions));

    out += &format!(
        "\tand |{}, {}",
        skillset_var(skillset),
        resources
            .iter()
            .map(|id| resource_var(skillset, skillset.get(*id).unwrap()))
            .fold(skill_var(skillset, skill), |acc, res| format!(
                "{}, {}",
                acc, res
            ))
    );
    if label {
        out += &format!(", {}", &trans_label_variable())
    }
    out += "|(\n";
    
    out += &format!(
        "\t\t{}' = Lock and {}' = Interrupted\n",
        skillset_var(skillset),
        skill_var(skillset, skill)
    );
    // Effects
    for effect in effects {
        out += &format!(
            "\t\tand {}' = {}\n",
            resource_var(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap()
            ),
            resource_state(
                skillset,
                skillset.get(effect.resource().resolved()).unwrap(),
                skillset.get(effect.state().resolved()).unwrap()
            )
        );
    }
    // Postconditions
    for post in postconditions.iter() {
        let expr = post.expr();
        out += &format!("\t\tand {}\n", next_expr_to_tatam(skillset, expr));
    }

    if label {
        out += &format!("\t\t\tand {}' = {}\n", &trans_label_variable(), &trans_label_enum_value(&format!( "{}_{}_interrupting_to_interrupted", skillset.name(), skill.name())));
    }

    out += "\t)\n";
    out += "}\n";

    ModelTransNames {
        model: out,
        transition_names: trans_names,
    }
}
