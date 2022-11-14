use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "cmd", content = "msg")]
#[serde(rename_all = "snake_case")]
pub(crate) enum ClientCommand {
    Play(usize),
    Hover(usize),
    Unhover(usize),
    GetTurnPlayer,
    GetGameState,
    GetTimers,
    Resign,
}

impl ClientCommand {
    pub(crate) fn check_valid_field(&self) -> bool {
        match self {
            Self::Play(x) | Self::Hover(x) | Self::Unhover(x) => x >= &0 && x < &9,
            _ => true,
        }
    }
}
