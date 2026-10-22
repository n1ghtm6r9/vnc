use winit::keyboard::Key;

pub fn character_of_key(key: &Key) -> Option<char> {
    let Key::Character(text) = key else {
        return None;
    };
    let mut characters = text.chars();
    let character = characters.next()?;
    characters.next().is_none().then_some(character)
}
