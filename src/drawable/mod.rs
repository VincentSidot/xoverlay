use x11rb::protocol::xproto::Drawable as XDrawable;

use crate::math::vec::Vec2;

pub mod pixmap;
pub mod window;

pub trait Drawable {
    fn id(&self) -> XDrawable;

    fn size(&self) -> Vec2<u16>;

    fn position(&self) -> Vec2<i16>;

    fn depth(&self) -> u8;

    fn width(&self) -> u16 {
        self.size().x
    }

    fn height(&self) -> u16 {
        self.size().y
    }

    fn x(&self) -> i16 {
        self.position().x
    }

    fn y(&self) -> i16 {
        self.position().y
    }
}
