use iced::{widget::Column, Length};
pub fn spacer<'a, T>() -> Column<'a, T> {
    Column::new().width(Length::Fill)
}
