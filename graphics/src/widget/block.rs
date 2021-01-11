//! Decorate/distribute content and apply alignment.

use iced_native::block;
use iced_native::mouse;
use crate::{Backend, Primitive, Renderer};
use iced_native::{Background, Color, Element, Layout, Point, Rectangle};

pub use iced_style::block::{Style, StyleSheet};

/// A container that distributes its contents vertically.
pub type Block<'a, Message, Backend> =
    iced_native::Block<'a, Message, Renderer<Backend>>;

impl<B> block::Renderer for Renderer<B>
where
    B: Backend,
{
    type Style = Box<dyn StyleSheet>;

    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        children: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        style: &Self::Style,
    ) -> Self::Output {
        let style = style.style();
        let mut mouse_interaction = mouse::Interaction::default();

        let (content, mouse_interaction) = (
            Primitive::Group {
                primitives: children
                    .iter()
                    .zip(layout.children())
                    .map(|(child, layout)| {
                        let (primitive, new_mouse_interaction) = child.draw(
                            self,
                            defaults,
                            layout,
                            cursor_position,
                            viewport,
                        );

                        if new_mouse_interaction > mouse_interaction {
                            mouse_interaction = new_mouse_interaction;
                        }

                        primitive
                    })
                    .collect(),
            },
            mouse_interaction,
        );

        if let Some(background) = background(layout.bounds(), &style) {
            (
                Primitive::Group {
                    primitives: vec![background, content],
                },
                mouse_interaction,
            )
        } else {
            (content, mouse_interaction)
        }
    }
}

pub(crate) fn background(
    bounds: Rectangle,
    style: &Style,
) -> Option<Primitive> {
    if style.background.is_some() || style.border_width > 0.0 {
        Some(Primitive::Quad {
            bounds,
            background: style
                .background
                .unwrap_or(Background::Color(Color::TRANSPARENT)),
            border_radius: style.border_radius,
            border_width: style.border_width,
            border_color: style.border_color,
        })
    } else {
        None
    }
}
