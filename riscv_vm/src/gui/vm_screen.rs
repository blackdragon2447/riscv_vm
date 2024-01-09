use iced::{
    widget::{
        canvas::{self, Cache, Geometry, Text},
        Canvas,
    },
    Color, Element, Font, Length, Point, Renderer, Size, Theme, Vector,
};

#[derive(Clone, Copy)]
pub struct VgaCharacter {
    char: char,
    foreground: Color,
    background: Color,
}

pub(super) struct VmScreen {
    vga_text_buf: [[Option<VgaCharacter>; 80]; 25],
    text_cache: Cache,
}

impl Default for VmScreen {
    fn default() -> Self {
        Self {
            vga_text_buf: [[None; 80]; 25],
            text_cache: Cache::default(),
        }
    }
}

#[derive(Default)]
pub enum State {
    #[default]
    VgaTextMode,
}

pub enum Message {}

impl VmScreen {
    pub fn new() -> Self {
        let mut inst = Self {
            vga_text_buf: [[None; 80]; 25],
            text_cache: Cache::default(),
        };
        inst
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self).width(80 * 9).height(25 * 16).into()
    }
}

impl canvas::Program<Message> for VmScreen {
    type State = State;

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: iced::Rectangle,
        cursor: iced::advanced::mouse::Cursor,
    ) -> Vec<Geometry> {
        match state {
            State::VgaTextMode => {
                let text = self.text_cache.draw(renderer, bounds.size(), |frame| {
                    frame.fill_rectangle(Point::ORIGIN, bounds.size(), Color::BLACK);
                    for (l, line) in self.vga_text_buf.iter().enumerate() {
                        for (p, ch) in line.iter().enumerate() {
                            if let Some(ch) = ch {
                                frame.translate(Vector {
                                    x: (p as f32) * 10.0,
                                    y: (l as f32) * 16.0,
                                });
                                frame.fill_text(Text {
                                    content: ch.char.to_string(),
                                    color: ch.foreground,
                                    size: 16.0,
                                    font: Font::MONOSPACE,
                                    ..Text::default()
                                });
                                frame.translate(Vector {
                                    x: (p as f32) * -10.0,
                                    y: (l as f32) * -16.0,
                                });
                                frame.fill_rectangle(
                                    Point::new(p as f32 * 10.0, l as f32 * 16.0),
                                    Size::new(10.0, 16.0),
                                    ch.background,
                                );
                            }
                        }
                    }
                });
                return vec![text.into()];
            }
        }
    }
}
