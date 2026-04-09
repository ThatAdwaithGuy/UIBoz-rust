use crossterm::{
    event,
    event::Event,
    terminal::{disable_raw_mode, enable_raw_mode},
};

pub fn getch() -> std::io::Result<event::KeyEvent> {
    let event = get_event()?;

    if let Event::Key(key) = event {
        Ok(key)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid event captured",
        ))
    }
}

pub fn get_event() -> std::io::Result<event::Event> {
    enable_raw_mode()?;
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Ok(key) = event::read() {
            disable_raw_mode()?;
            return Ok(key);
        } else {
            disable_raw_mode()?;
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid event captured",
            ));
        }
    } else {
        disable_raw_mode()?;
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Invalid event captured",
        ));
    }
}
