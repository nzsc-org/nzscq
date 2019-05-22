use super::ArsenalItem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DequeueChoice {
    DrainAndExit(ArsenalItem),
    JustExit,
    Decline,
}
