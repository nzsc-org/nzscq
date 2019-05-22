use super::ArsenalItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DequeueChoice {
    DrainAndExit(ArsenalItem),
    JustExit,
    Decline,
}
