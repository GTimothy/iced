use crate::{Backend, Primitive, Renderer};
use iced_native::layer;
use iced_native::mouse;
use iced_native::{Element, Layout, Point, Rectangle};

/// A layer to layer content.
pub type Layer<'a, Message, Backend> =
    iced_native::Layer<'a, Message, Renderer<Backend>>;

impl<B> layer::Renderer for Renderer<B>
where
    B: Backend,
{
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        content: &Element<'_, Message, Self>,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Self::Output {
        content.draw(
            self,
            &defaults,
            layout,
            cursor_position,
            viewport,
        )
    }
}
