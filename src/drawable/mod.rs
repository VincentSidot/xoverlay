use x11rb::protocol::xproto::Drawable as XDrawable;

pub mod pixmap;
pub mod window;

pub trait Drawable {
    fn id(&self) -> XDrawable;

    fn width(&self) -> u16;
    fn height(&self) -> u16;

    fn x(&self) -> i16;
    fn y(&self) -> i16;

    fn depth(&self) -> u8;
}
