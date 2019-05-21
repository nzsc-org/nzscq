use super::ArsenalItem;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DequeueChoice {
    DrainAndExit(ArsenalItem),
    JustExit,
    Decline,
}