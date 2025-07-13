fn main() -> iced::Result {
    dotenvy::from_filename("kyefa-desktop/.env").ok();
    kyefa_desktop::app::run()
}
