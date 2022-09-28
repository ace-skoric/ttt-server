pub(crate) enum ClientCommand {
    Play(usize),    // 0 x
    Hover(usize),   // 1
    Unhover(usize), // 2
    GetTurnPlayer,  // 3
    GetGameState,   // 4
    GetTimers,      // 5
    Resign,         // 6
    Error(String),  // 9
}

impl ClientCommand {
    pub(crate) fn parse_command(command: String) -> Self {
        let command = command
            .split_ascii_whitespace()
            .map(|c| c.parse::<usize>().unwrap_or(9))
            .collect::<Vec<usize>>();
        let size = command.len();
        let mut cmd: Self = Self::Error("Non existent command".to_string());
        if command[0] >= 3 && command[0] < 7 {
            if size > 1 {
                cmd = Self::Error("Too many arguments for this command".to_string());
            } else {
                match command[0] {
                    3 => cmd = Self::GetTurnPlayer,
                    4 => cmd = Self::GetGameState,
                    5 => cmd = Self::GetTimers,
                    6 => cmd = Self::Resign,
                    _ => (),
                }
            }
        }
        if command[0] == 0 || command[0] == 1 || command[0] == 2 {
            if size == 1 {
                cmd = Self::Error("Too few arguments for this command".to_string());
            } else if size > 2 {
                cmd = Self::Error("Too many arguments for this command".to_string());
            } else if command[1] >= 9 {
                cmd = Self::Error("Non existent board space".to_string());
            } else {
                match command[0] {
                    0 => cmd = Self::Play(command[1]),
                    1 => cmd = Self::Hover(command[1]),
                    2 => cmd = Self::Unhover(command[1]),
                    _ => (),
                }
            }
        }
        cmd
    }
}
