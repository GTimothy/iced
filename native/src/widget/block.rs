//! Decorate and distributes content horizontally/vertically and apply alignment and spacing.
use crate::event::{self, Event};
use crate::layout::{self, flex};
use crate::overlay;
use crate::{
    Align, Clipboard, Element, Hasher, Layout, Length, Point, Rectangle, Widget,
};

use std::hash::Hash;
use std::u32;

/// A container that distributes its contents horizontally/vertically.
#[allow(missing_debug_implementations)]
pub struct Block<'a, Message, Renderer: self::Renderer, > {
    spacing: u16,
    padding: u16,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    horizontal_alignment: Align,
    vertical_alignment: Align,
    style: Renderer::Style,
    children: Vec<Element<'a, Message, Renderer>>,
    flex_axis: flex::Axis
}

impl<'a, Message, Renderer> Block<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// Creates an empty [`Block`].
    pub fn new(flex_axis: flex::Axis) -> Self {
        Self::with_children(Vec::new(), flex_axis)
    }

    /// Creates a [`Column`] with the given elements.
    pub fn with_children( children: Vec<Element<'a, Message, Renderer>>, flex_axis: flex::Axis) -> Self {
        Block {
            spacing: 0,
            padding: 0,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            horizontal_alignment: Align::Start,
            vertical_alignment: Align::Start,
            style: Renderer::Style::default(),
            children,
            flex_axis
        }
    }
    /// Sets the vertical spacing _between_ elements.
    ///
    /// Custom margins per element do not exist in Iced. You should use this
    /// method instead! While less flexible, it helps you keep spacing between
    /// elements consistent.
    pub fn spacing(mut self, units: u16) -> Self {
        self.spacing = units;
        self
    }

    /// Sets the padding of the [`Block`].
    pub fn padding(mut self, units: u16) -> Self {
        self.padding = units;
        self
    }

    /// Sets the width of the [`Block`].
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`Block`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`Block`].
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`Block`] in pixels.
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_items(mut self, alignment: Align) -> Self {
        match self.flex_axis {
            crate::Axis::Horizontal => self.vertical_alignment = alignment,
            crate::Axis::Vertical => self.horizontal_alignment = alignment
        };
        self
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_x(mut self, alignment: Align) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Container`].
    pub fn align_y(mut self, alignment: Align) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Sets the flex axis of the [`Block`] .
    pub fn flex_axis(mut self, flex_axis: flex::Axis) -> Self {
        self.flex_axis = flex_axis;
        self
    }

    /// Centers the contents in the horizontal axis of the [`Container`].
    pub fn center_x(mut self) -> Self {
        self.horizontal_alignment = Align::Center;
        self
    }

    /// Centers the contents in the vertical axis of the [`Container`].
    pub fn center_y(mut self) -> Self {
        self.vertical_alignment = Align::Center;
        self
    }

    /// Sets the style of the [`Container`].
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }

    /// Adds an element to the [`Block`].
    pub fn push<E>(mut self, child: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        self.children.push(child.into());
        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer>
    for Block<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
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
        let padding = f32::from(self.padding);

        let limits = limits
	    .loose()
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let align_item = match self.flex_axis {
            crate::Axis::Horizontal => self.vertical_alignment,
            crate::Axis::Vertical => self.horizontal_alignment
        };
        let mut l = layout::flex::resolve(
            self.flex_axis,
            renderer,
            &limits,
            self.padding as f32,
            self.spacing as f32,
            align_item,
            &self.children,
        );
        l.align(self.horizontal_alignment, self.vertical_alignment, limits.resolve(l.size()));
        l
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
        self.children
            .iter_mut()
            .zip(layout.children())
            .map(|(child, layout)| {
                child.widget.on_event(
                    event.clone(),
                    layout,
                    cursor_position,
                    messages,
                    renderer,
                    clipboard,
                )
            })
            .fold(event::Status::Ignored, event::Status::merge)
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
            &self.children,
            layout,
            cursor_position,
            viewport,
            &self.style,
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        struct Marker;
        std::any::TypeId::of::<Marker>().hash(state);

        self.padding.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        self.max_width.hash(state);
        self.max_height.hash(state);
        // self.align_items.hash(state);
        // self.spacing.hash(state);
        // self.padding.hash(state);

        for child in &self.children {
            child.widget.hash_layout(state);
        }
    }

    fn overlay(
        &mut self,
        layout: Layout<'_>,
    ) -> Option<overlay::Element<'_, Message, Renderer>> {
        self.children
            .iter_mut()
            .zip(layout.children())
            .filter_map(|(child, layout)| child.widget.overlay(layout))
            .next()
    }
}

/// The renderer of a [`Block`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`Block`] in your user interface.
///
/// [renderer]: crate::renderer
pub trait Renderer: crate::Renderer + Sized{
    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`Block`].
    ///
    /// It receives:
    /// - the children of the [`Block`]
    /// - the [`Layout`] of the [`Block`] and its children
    /// - the cursor position
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        children: &[Element<'_, Message, Self>],
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        style: &Self::Style,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<Block<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'a + self::Renderer,
    Message: 'a,
{
    fn from(
        block: Block<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(block)
    }
}
