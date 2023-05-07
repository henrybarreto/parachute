/// Represents valid actions on Parachute server.
pub enum Action {
    /// A download action.
    DOWNLOAD = 1,
    /// A upload action.
    UPLOAD = 2,
    /// A unknown action.
    UNKNOWN = 3,
}

impl Action {
    /// Converts a u8 to an Action.
    pub fn from_u8(value: u8) -> Action {
        match value {
            1 => Action::DOWNLOAD,
            2 => Action::UPLOAD,
            _ => Action::UNKNOWN,
        }
    }

    /// Converts an buffer with one byte to an Action.
    pub fn from_buffer(buffer: Vec<u8>) -> Action {
        Self::from_u8(buffer[0])
    }

    pub fn to_ne_bytes(action: Action) -> [u8; 1] {
        [action as u8]
    }
}
