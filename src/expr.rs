use super::*;
use naming::*;
use rl_model::model::expr::*;

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
