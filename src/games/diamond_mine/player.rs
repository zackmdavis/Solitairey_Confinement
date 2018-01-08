use std::collections::HashSet;

use super::{DiamondMine, Action};


pub struct Player {
    done: HashSet<Action>
}

impl Player {
    pub fn new() -> Self {
        Self { done: HashSet::new() }
    }

    pub fn select_action(&mut self, actions: Vec<Action>) -> Option<Action> {
        let (mut foundationals, tableauans) = actions.iter().cloned()
            .partition::<Vec<_>, _>(|action| match *action {
                Action::ToFoundation { .. } => true,
                Action::OnTableau { .. } => false
            });

        println!("{:?} {:?}", foundationals, tableauans);

        for tableau_action in tableauans {
            if self.done.get(&tableau_action).is_some() {
                continue;
            } else {
                self.done.insert(tableau_action.clone());
                return Some(tableau_action);
            }
        }

        if !foundationals.is_empty() {
            foundationals.sort_by_key(|ref action| match **action {
                Action::ToFoundation { card, .. } => { -card.value.as_int() },
                _ => { panic!("this can't be happening"); }
            });
            Some(foundationals.pop().unwrap())
        } else {
            None
        }
    }
}
