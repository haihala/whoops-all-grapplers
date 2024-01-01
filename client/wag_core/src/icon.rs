use bevy::utils::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Icon {
    ThumbTacks(usize),
}
impl Icon {
    pub fn paths() -> HashMap<Icon, String> {
        (1..9)
            .map(|id| {
                (
                    Icon::ThumbTacks(id),
                    format!("icons/thumbtack{}.png", usize::pow(2, (id - 1) as u32)),
                )
            })
            .collect()
    }
}
