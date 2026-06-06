//! Search parameters — controls how long and how deep to search.
//!
//! Parsed from UCI `go` commands. Used to calculate the time
//! available for the next move.

use std::time::Duration;

use crate::piece::Color;

/// Parameters controlling the search, typically received from a UCI `go` command.
#[derive(Debug, Default)]
pub struct SearchParameters {
    /// White's remaining time in milliseconds.
    pub wtime: Option<u64>,
    /// Black's remaining time in milliseconds.
    pub btime: Option<u64>,
    /// White's increment per move in milliseconds.
    pub winc: Option<u64>,
    /// Black's increment per move in milliseconds.
    pub binc: Option<u64>,
    /// Number of moves until the next time control.
    pub movestogo: Option<u64>,
    /// Search to this depth only.
    pub depth: Option<u8>,
    /// Search for exactly this many milliseconds.
    pub movetime: Option<u64>,
    /// Search until explicitly stopped.
    pub infinite: bool,
}

impl SearchParameters {
    /// Calculates how long the engine should spend on the next move.
    ///
    /// Returns `None` if the search should run indefinitely (e.g. `infinite` mode
    /// or no time information available).
    pub fn time_for_move(&self, side_to_move: Color) -> Option<Duration> {
        if self.infinite {
            return None;
        }

        if let Some(mt) = self.movetime {
            return Some(Duration::from_millis(mt));
        }

        let time = match side_to_move {
            Color::White => self.wtime?,
            Color::Black => self.btime?,
        };

        let inc = match side_to_move {
            Color::White => self.winc.unwrap_or(0),
            Color::Black => self.binc.unwrap_or(0),
        };

        let moves_to_go = self.movestogo.unwrap_or(30);

        Some(Duration::from_millis(time / moves_to_go + inc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infinite_returns_none() {
        let params = SearchParameters {
            infinite: true,
            ..Default::default()
        };
        assert!(params.time_for_move(Color::White).is_none());
    }

    #[test]
    fn no_time_info_returns_none() {
        assert!(
            SearchParameters::default()
                .time_for_move(Color::White)
                .is_none()
        );
    }

    #[test]
    fn movetime_is_used_directly() {
        let params = SearchParameters {
            movetime: Some(5000),
            ..Default::default()
        };
        assert_eq!(
            params.time_for_move(Color::White),
            Some(Duration::from_millis(5000))
        );
    }

    #[test]
    fn time_with_increment_and_movestogo() {
        let params = SearchParameters {
            wtime: Some(30000),
            winc: Some(1000),
            movestogo: Some(10),
            ..Default::default()
        };
        assert_eq!(
            params.time_for_move(Color::White),
            Some(Duration::from_millis(4000))
        );
    }
}
