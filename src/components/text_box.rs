use ratatui::{buffer::Buffer, layout::{Alignment, Constraint, Flex, Layout, Margin, Rect}, text::Line, widgets::{block::Title, Block, BorderType, Widget}};

#[derive(Debug, Default, Clone)]
pub struct TextBoxWidget<'a> {
    border_block: Option<Block<'a>>,
    constraints: Option<Vec<Constraint>>,
    content: Vec<Line<'a>>,
    flex: Flex,
    title: Option<Title<'a>>,
}

impl<'a> TextBoxWidget<'a> {
    #[inline]
    pub fn new<C>(content: C) -> Self
    where
        C: IntoIterator,
        C::Item: Into<Line<'a>> + Widget,
        Vec<Line<'a>>: From<C>,
    {
        TextBoxWidget {
            border_block: None,
            constraints: None,
            content: content.into(),
            flex: Flex::SpaceAround,
            title: None,
        }
    }

    #[inline]
    pub fn bordered<C>(content: C) -> Self
    where
        C: IntoIterator,
        C::Item: Into<Line<'a>> + Widget,
        Vec<Line<'a>>: From<C>,
    {
        let mut text_box = Self::new(content);
        text_box.border_block = Some(Block::bordered().border_type(BorderType::Rounded));
        text_box
    }

    #[inline]
    pub fn border_block(mut self, border_block: Block<'a>) -> Self {
        self.border_block = Some(border_block);
        self
    }

    #[inline]
    pub fn constraints<I>(mut self, constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
        Vec<Constraint>: From<I>,
    {
        self.constraints = Some(constraints.into());
        self
    }

    #[inline]
    pub fn content<C>(mut self, content: C) -> Self
    where
        C: IntoIterator,
        C::Item: Into<Line<'a>> + Widget,
        Vec<Line<'a>>: From<C>,
    {
        self.content = content.into();
        self
    }

    #[inline]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    #[inline]
    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Title<'a>>,
    {
        self.title = Some(title.into());
        self
    }
}

// TODO: Re-implement using ratatui-image.

impl<'a> Widget for TextBoxWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut inner_area = area;

        if self.border_block.is_some() {
            let mut border_block = self.border_block.unwrap();

            if self.title.is_some() {
                border_block = border_block.title(self.title.unwrap().alignment(Alignment::Center));
            }

            border_block.render(area, buf);
            inner_area = area.inner(Margin::new(1, 1));
        }

        let areas = Layout::vertical(
            self.constraints.unwrap_or(
                vec![Constraint::Length(1); self.content.clone().into_iter().len()]
            )
        ).flex(self.flex).split(inner_area);
        for (idx, line) in self.content.clone().into_iter().enumerate() {
            line.render(areas[idx], buf);
        }
    }
}
