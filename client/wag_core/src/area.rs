use bevy::{math::Rect, prelude::*};

#[derive(Clone, Copy, Default, Debug, Reflect, PartialEq)]
pub struct Area {
    center: Vec2,
    width: f32,
    height: f32,
}

impl Area {
    pub fn of_size(width: f32, height: f32) -> Self {
        Self::new(0.0, 0.0, width, height)
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        assert!(width >= 0.0);
        assert!(height >= 0.0);
        Self {
            center: Vec2::new(x, y),
            width,
            height,
        }
    }
    pub fn from_sides(top: f32, bottom: f32, left: f32, right: f32) -> Self {
        Self {
            center: Vec2::new((left + right) / 2.0, (top + bottom) / 2.0),
            width: right - left,
            height: top - bottom,
        }
    }
    pub fn from_center_size(center: Vec2, size: Vec2) -> Self {
        Self {
            center,
            width: size.x,
            height: size.y,
        }
    }

    pub fn center(&self) -> Vec2 {
        self.center
    }
    pub fn width(&self) -> f32 {
        self.width
    }
    pub fn height(&self) -> f32 {
        self.height
    }
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.width(), self.height())
    }

    // Utilities
    pub fn with_center(self, new_center: Vec2) -> Self {
        Self::from_center_size(new_center, self.size())
    }
    pub fn with_offset(self, new_origin: Vec2) -> Self {
        Self::from_center_size(self.center + new_origin, self.size())
    }
    pub fn intersection(&self, other: &Area) -> Option<Area> {
        let x_overlap = self.left() < other.right() && self.right() > other.left();
        let y_overlap = self.bottom() < other.top() && self.top() > other.bottom();

        if x_overlap && y_overlap {
            Some(Self::from_sides(
                self.top().min(other.top()),
                self.bottom().max(other.bottom()),
                self.left().max(other.left()),
                self.right().min(other.right()),
            ))
        } else {
            None
        }
    }
    pub fn intersects(&self, other: &Area) -> bool {
        self.intersection(other).is_some()
    }

    // For conversions
    pub fn top(&self) -> f32 {
        self.center.y + (self.height / 2.0)
    }
    pub fn bottom(&self) -> f32 {
        self.center.y - (self.height / 2.0)
    }
    pub fn right(&self) -> f32 {
        self.center.x + (self.width / 2.0)
    }
    pub fn left(&self) -> f32 {
        self.center.x - (self.width / 2.0)
    }
}

#[allow(clippy::from_over_into)]
impl Into<Rect> for Area {
    fn into(self) -> Rect {
        Rect {
            max: Vec2::new(self.left(), self.bottom()),
            min: Vec2::new(self.right(), self.top()),
        }
    }
}
impl From<Rect> for Area {
    fn from(rect: Rect) -> Self {
        Self::from_sides(rect.max.y, rect.min.y, rect.min.x, rect.max.y)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn collides_with_self() {
        let area = Area::new(0.0, 0.0, 1.0, 1.0);
        assert!(area.intersects(&area));
        assert!(area == area.intersection(&area).unwrap());
    }

    #[test]
    fn no_collision() {
        let area1 = Area::new(-1.0, 0.0, 1.0, 1.0);
        let area2 = Area::new(1.0, 0.0, 1.0, 1.0);
        assert!(!area1.intersects(&area2));
        assert!(area1.intersection(&area2).is_none());
    }

    #[test]
    fn corner_collision() {
        let area1 = Area::new(0.0, 0.0, 2.0, 2.0);
        let area2 = Area::new(1.0, 1.0, 2.0, 2.0);
        let overlap = Area::from_sides(1.0, 0.0, 0.0, 1.0);
        assert!(area1.intersects(&area2));
        assert!(area1.intersection(&area2).unwrap() == overlap);
    }

    #[test]
    fn full_overlap() {
        let area1 = Area::new(0.0, 0.0, 10.0, 10.0);
        let area2 = Area::new(1.0, 1.0, 1.0, 1.0);
        assert!(area1.intersects(&area2));
        assert!(area1.intersection(&area2).unwrap() == area2);
    }

    #[test]
    fn edge_overlap() {
        let area1 = Area::new(-0.5, 0.0, 1.0, 1.0);
        let area2 = Area::new(0.5, 0.0, 1.0, 1.0);
        assert!(!area1.intersects(&area2));
    }

    #[test]
    fn constructors_match() {
        let new = Area::new(1.0, 2.0, 3.0, 4.0);
        let sides = Area::from_sides(4.0, 0.0, -0.5, 2.5);
        let centersize = Area::from_center_size(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));

        assert!(new == sides);
        assert!(new == centersize);
        assert!(sides == centersize);
    }
}
