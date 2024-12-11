use bevy::prelude::*;
use foundation::Player;

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

#[derive(Debug)]
pub struct SharedVerticalNav {
    pub p1_select: VerticalMenuNavigation,
    pub p1_locked: bool,
    pub p2_select: VerticalMenuNavigation,
    pub p2_locked: bool,
}

impl SharedVerticalNav {
    pub fn up(&mut self, player: Player) {
        match player {
            Player::One => {
                if !self.p1_locked {
                    self.p1_select.up()
                }
            }
            Player::Two => {
                if !self.p2_locked {
                    self.p2_select.up()
                }
            }
        }
    }

    pub fn down(&mut self, player: Player) {
        match player {
            Player::One => {
                if !self.p1_locked {
                    self.p1_select.down()
                }
            }
            Player::Two => {
                if !self.p2_locked {
                    self.p2_select.down()
                }
            }
        }
    }

    pub fn locked(&self, player: Player) -> bool {
        match player {
            Player::One => self.p1_locked,
            Player::Two => self.p2_locked,
        }
    }

    pub fn both_locked(&self) -> bool {
        self.p1_locked && self.p2_locked
    }

    pub fn lock_in(&mut self, player: Player) {
        match player {
            Player::One => self.p1_locked = true,
            Player::Two => self.p2_locked = true,
        }
    }

    pub fn unlock(&mut self, player: Player) {
        match player {
            Player::One => self.p1_locked = false,
            Player::Two => self.p2_locked = false,
        }
    }

    pub fn selected(&self, player: Player) -> Entity {
        match player {
            Player::One => self.p1_select.selected,
            Player::Two => self.p2_select.selected,
        }
    }
}
