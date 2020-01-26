//! Allow your users to perform actions by pressing a hover_area.
//!
//! A [`HoverArea`] has some local [`State`].
//!
//! [`HoverArea`]: struct.HoverArea.html
//! [`State`]: struct.State.html
use crate::{
    input::{mouse, ButtonState},
    layout, Clipboard, Element, Event, Hasher, Layout, Length, Point,
    Rectangle, Widget,
};
use std::hash::Hash;

/// A generic widget that produces a message when pressed.
///
/// ```
/// # use iced_native::{hover_area, Text};
/// #
/// # type HoverArea<'a, Message> =
/// #     iced_native::HoverArea<'a, Message, iced_native::renderer::Null>;
/// #
/// enum Message {
///     ButtonPressed,
/// }
///
/// let mut state = hover_area::State::new();
/// let hover_area = HoverArea::new(&mut state, Text::new("Press me!"))
///     .on_press(Message::ButtonPressed);
/// ```
#[allow(missing_debug_implementations)]
pub struct HoverArea<'a, Message, Renderer: self::Renderer> {
    state: &'a mut State,
    content: Element<'a, Message, Renderer>,
    on_hover: Option<Message>,
    width: Length,
    height: Length,
    min_width: u32,
    min_height: u32,
    padding: u16,
    style: Renderer::Style,
}

impl<'a, Message, Renderer> HoverArea<'a, Message, Renderer>
where
    Renderer: self::Renderer,
{
    /// Creates a new [`HoverArea`] with some local [`State`] and the given
    /// content.
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    /// [`State`]: struct.State.html
    pub fn new<E>(state: &'a mut State, content: E) -> Self
    where
        E: Into<Element<'a, Message, Renderer>>,
    {
        HoverArea {
            state,
            content: content.into(),
            on_hover: None,
            width: Length::Shrink,
            height: Length::Shrink,
            min_width: 0,
            min_height: 0,
            padding: Renderer::DEFAULT_PADDING,
            style: Renderer::Style::default(),
        }
    }

    /// Sets the width of the [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the minimum width of the [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn min_width(mut self, min_width: u32) -> Self {
        self.min_width = min_width;
        self
    }

    /// Sets the minimum height of the [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn min_height(mut self, min_height: u32) -> Self {
        self.min_height = min_height;
        self
    }

    /// Sets the padding of the [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn padding(mut self, padding: u16) -> Self {
        self.padding = padding;
        self
    }

    /// Sets the message that will be produced when the [`HoverArea`] is
    /// hovered.
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn on_hover(mut self, msg: Message) -> Self {
        self.on_hover = Some(msg);
        self
    }

    /// Sets the style of the [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    pub fn style(mut self, style: impl Into<Renderer::Style>) -> Self {
        self.style = style.into();
        self
    }
}

/// The local state of a [`HoverArea`].
///
/// [`HoverArea`]: struct.HoverArea.html
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct State {
    is_hovered: bool,
}

impl State {
    /// Creates a new [`State`].
    ///
    /// [`State`]: struct.State.html
    pub fn new() -> State {
        State::default()
    }
}
impl<'a, Message, Renderer> Widget<Message, Renderer>
    for HoverArea<'a, Message, Renderer>
where
    Renderer: self::Renderer,
    Message: Clone,
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
            .min_width(self.min_width)
            .min_height(self.min_height)
            .width(self.width)
            .height(self.height)
            .pad(padding);

        let mut content = self.content.layout(renderer, &limits);

        content.bounds.x = padding;
        content.bounds.y = padding;

        let size = limits.resolve(content.size()).pad(padding);

        layout::Node::with_children(size, vec![content])
    }

    fn on_event(
        &mut self,
        _event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) {
        let bounds = layout.bounds();
        let old_value = self.state.is_hovered;
        let new_value = bounds.contains(cursor_position);
        self.state.is_hovered = new_value;
        if new_value && new_value != old_value {
            if let Some(on_hover) = self.on_hover.clone() {
                messages.push(on_hover);
            }
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        defaults: &Renderer::Defaults,
        layout: Layout<'_>,
        cursor_position: Point,
    ) -> Renderer::Output {
        renderer.draw(
            defaults,
            layout.bounds(),
            cursor_position,
            self.on_hover.is_none(),
            self.state.is_hovered,
            &self.style,
            &self.content,
            layout.children().next().unwrap(),
        )
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.width.hash(state);
        self.content.hash_layout(state);
    }
}

/// The renderer of a [`HoverArea`].
///
/// Your [renderer] will need to implement this trait before being
/// able to use a [`HoverArea`] in your user interface.
///
/// [`HoverArea`]: struct.HoverArea.html
/// [renderer]: ../../renderer/index.html
pub trait Renderer: crate::Renderer + Sized {
    /// The default padding of a [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    const DEFAULT_PADDING: u16;

    /// The style supported by this renderer.
    type Style: Default;

    /// Draws a [`HoverArea`].
    ///
    /// [`HoverArea`]: struct.HoverArea.html
    fn draw<Message>(
        &mut self,
        defaults: &Self::Defaults,
        bounds: Rectangle,
        cursor_position: Point,
        is_disabled: bool,
        is_pressed: bool,
        style: &Self::Style,
        content: &Element<'_, Message, Self>,
        content_layout: Layout<'_>,
    ) -> Self::Output;
}

impl<'a, Message, Renderer> From<HoverArea<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Renderer: 'static + self::Renderer,
    Message: 'static + Clone,
{
    fn from(
        hover_area: HoverArea<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new(hover_area)
    }
}
