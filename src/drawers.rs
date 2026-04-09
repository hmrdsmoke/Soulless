use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Color, Element, Length, Padding};
use crate::search::Message as SearchMessage;

pub fn view(search: &crate::search::Search) -> Element<'_, SearchMessage> {
    let header = container(
        text("Soulless")
            .size(32)
            .style(|_| iced::widget::text::Style {
                color: Some(Color::from_rgb(1.0, 0.95, 0.0)),
                ..Default::default()
            })
    )
    .padding(20)
    .center_x(Length::Fill)
    .style(|_| container::Style {
        background: Some(iced::color!(0x1e, 0x1e, 0x1e).into()),
        border: iced::border::rounded(8),
        ..Default::default()
    });

    let search_bar = text_input("Type to search...", &search.query)
        .on_input(SearchMessage::QueryChanged)
        .padding(14)
        .size(18);

    let results = scrollable(
        column(
            search.filtered_apps().into_iter().map(|(_, exec)| {
                button(
                    row![]
                        .spacing(16)
                        .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .padding(14)
                .on_press(SearchMessage::AppClicked(exec))
                .into()
            })
        )
        .spacing(4)
        .padding(Padding::new(8.0))
    )
    .height(Length::Fill);

    column![
        header,
        container(search_bar).padding(16),
        results
    ]
    .spacing(8)
    .width(Length::Fixed(460.0))
    .height(Length::Fill)
    .into()
}

// === YOUR ORIGINAL COMMENTS (preserved exactly) ===
 // Status is now an enum in iced 0.14 :: MRV