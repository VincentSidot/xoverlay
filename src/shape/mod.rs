#![allow(dead_code)]
use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::Gcontext};

use crate::{drawable::Drawable, Color};

mod arc;
pub mod coord;
mod rectangle;

pub use arc::Arc;
pub use rectangle::Rectangle;

pub trait Shape<C>
where
    C: Connection,
{
    fn draw(&self, conn: &C, gc: &Gcontext, drawable: &dyn Drawable) -> Result<(), Box<dyn Error>>;

    fn color(&self) -> &Color;
}
