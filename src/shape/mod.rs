#![allow(dead_code)]
use std::error::Error;
use x11rb::{connection::Connection, protocol::xproto::Gcontext};

use crate::{
    drawable::Drawable,
    Color
};

mod rectangle;
mod arc;

pub use rectangle::Rectangle;
pub use arc::Arc;

pub trait Shape<C>
where
    C: Connection
{
    fn draw(&self, conn: &C, gc: &Gcontext, drawable: &dyn Drawable) ->Result<(), Box<dyn Error>>;

    fn color(&self) -> &Color;
}