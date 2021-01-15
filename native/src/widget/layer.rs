//! Layer content.
use std::hash::Hash;

use crate::event::{self, Event};
use crate::layout;
use crate::overlay;
use crate::{
    Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

use std::u32;

/// An element decorating some content.
///
/// It is normally  used to create superposable layers of content.
#[allow(missing_debug_implementations)]
pub struct Layer<'a, Message, Renderer> {
    layer_num: i32,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    content: Element<'a, Message, Renderer>,
}

impl<'a, Message, Renderer> Layer<'a, Message, Renderer>
{
    /// Creates an empty [`Layer`].
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Layer {
            layer_num: 1,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            content: content.into(),
        }
    }
    /// Sets the layer number of the [`Layer`]. By default, Widgets behave like they are on layer
    /// number 0.
    pub fn layer_num(mut self, layer_num: i32) -> Self {
        self.layer_num = layer_num;
        self
    }

    /// Sets the width of the [`Layer`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Layer`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Layer`].
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Layer`] in pixels.
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Layer<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    fn layer(&self) -> i32 {
        self.layer_num
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let content = self.content.layout(renderer, limits);
        let size = limits.resolve(content.size());

        layout::Node::with_children(size, vec![content])
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        renderer: &Renderer,
        clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        self.content.widget.on_event(
            event,
            layout.children().next().unwrap(),
            cursor_position,
            messages,
            renderer,
            clipboard,
        )
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            &self.content,
            layout,
            cursor_position,
            viewport
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.layer_num.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);

        self.content.hash_layout(state);
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
    ) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.content.overlay(layout.children().next().unwrap())
    }
}

/// The renderer of a [`Layer`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Layer`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer: crate::Renderer + Sized {

    /// Draws a [`Layer`].
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        content: &Element<'_, Message, Self>,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Layer<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        layer: Layer<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(layer)
    }
}
