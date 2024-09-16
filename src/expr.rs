use std::collections::HashSet;

use super::*;
use naming::*;
// use rl_model::model::expr::*;

pub fn expr_to_tatam(skillset: &Skillset, expr: &Expr) -> String {
    match expr {
        Expr::True => "true".into(),
        Expr::False => "false".into(),
        Expr::ResourceEq(id, state) => {
            let resource = skillset.get(id.resolved()).unwrap();
            let state = skillset.get(state.resolved()).unwrap();
            format!(
                "({} = {})",
                resource_var(skillset, resource),
                resource_state(skillset, state)
            )
        }
        Expr::ResourceNe(id, state) => {
            let resource = skillset.get(id.resolved()).unwrap();
            let state = skillset.get(state.resolved()).unwrap();
            format!(
                "({} != {})",
                resource_var(skillset, resource),
                resource_state(skillset, state)
            )
        }
        Expr::Not(e) => format!("(not {})", expr_to_tatam(skillset, e)),
        Expr::And(l, r) => format!(
            "({} and {})",
            expr_to_tatam(skillset, l),
            expr_to_tatam(skillset, r)
        ),
        Expr::Or(l, r) => format!(
            "({} or {})",
            expr_to_tatam(skillset, l),
            expr_to_tatam(skillset, r)
        ),
        Expr::Implies(l, r) => format!(
            "({} implies {})",
            expr_to_tatam(skillset, l),
            expr_to_tatam(skillset, r)
        ),
    }
}

pub fn next_expr_to_tatam(skillset: &Skillset, expr: &Expr) -> String {
    match expr {
        Expr::True => "true".into(),
        Expr::False => "false".into(),
        Expr::ResourceEq(id, state) => {
            let resource = skillset.get(id.resolved()).unwrap();
            let state = skillset.get(state.resolved()).unwrap();
            format!(
                "({}' = {})",
                resource_var(skillset, resource),
                resource_state(skillset, state)
            )
        }
        Expr::ResourceNe(id, state) => {
            let resource = skillset.get(id.resolved()).unwrap();
            let state = skillset.get(state.resolved()).unwrap();
            format!(
                "({}' != {})",
                resource_var(skillset, resource),
                resource_state(skillset, state)
            )
        }
        Expr::Not(e) => format!("(not {})", next_expr_to_tatam(skillset, e)),
        Expr::And(l, r) => format!(
            "({} and {})",
            next_expr_to_tatam(skillset, l),
            next_expr_to_tatam(skillset, r)
        ),
        Expr::Or(l, r) => format!(
            "({} or {})",
            next_expr_to_tatam(skillset, l),
            next_expr_to_tatam(skillset, r)
        ),
        Expr::Implies(l, r) => format!(
            "({} implies {})",
            next_expr_to_tatam(skillset, l),
            next_expr_to_tatam(skillset, r)
        ),
    }
}

pub fn expr_resources(expr: &Expr) -> HashSet<ResourceId> {
    match expr {
        Expr::True => HashSet::default(),
        Expr::False => HashSet::default(),
        Expr::ResourceEq(r, _) => {
            let mut resources = HashSet::default();
            resources.insert(r.resolved());
            resources
        }
        Expr::ResourceNe(r, _) => {
            let mut resources = HashSet::default();
            resources.insert(r.resolved());
            resources
        }
        Expr::Not(e) => expr_resources(e),
        Expr::And(l, r) => {
            let mut resources = HashSet::default();
            resources.extend(expr_resources(l));
            resources.extend(expr_resources(r));
            resources
        }
        Expr::Or(l, r) => {
            let mut resources = HashSet::default();
            resources.extend(expr_resources(l));
            resources.extend(expr_resources(r));
            resources
        }
        Expr::Implies(l, r) => {
            let mut resources = HashSet::default();
            resources.extend(expr_resources(l));
            resources.extend(expr_resources(r));
            resources
        }
    }
}
