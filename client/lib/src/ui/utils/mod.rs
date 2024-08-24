use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct VerticalMenuNavigation {
    pub selected: Entity,
    targets: Vec<Entity>,
}
impl VerticalMenuNavigation {
    pub fn from_buttons(buttons: Vec<Entity>) -> Self {
        Self {
            selected: buttons[0],
            targets: buttons,
        }
    }

    fn selected_index(&self) -> usize {
        self.targets
            .iter()
            .position(|btn| *btn == self.selected)
            .unwrap()
    }

    pub fn up(&mut self) {
        let si = self.selected_index();
        self.selected = if si == 0 {
            *self.targets.last().unwrap()
        } else {
            self.targets[si - 1]
        }
    }

    pub fn down(&mut self) {
        let si = self.selected_index();
        self.selected = if si == self.targets.len() - 1 {
            *self.targets.first().unwrap()
        } else {
            self.targets[si + 1]
        }
    }
}
