use rust_fsm::*;

state_machine! {
    NaiveConsensus(Rand)

    Rand => {
        RandBlue => Blue [HighlightBlue],
        RandRed => Red [HighlightRed],
    },
    Blue => {
        MajorityBlue => Blue,
        MajorityRed => Red [HighlightRed],
        Standstill => Rand [ClearHighlight],
    },
    Red => {
        MajorityBlue => Blue [HighlightBlue],
        MajorityRed => Red,
        Standstill => Rand [ClearHighlight],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut machine: StateMachine<NaiveConsensus> = StateMachine::new();
        let _ = machine.consume(&NaiveConsensusInput::RandBlue);
        assert_eq!(machine.state(), &NaiveConsensusState::Blue);

        let output = machine.consume(&NaiveConsensusInput::MajorityRed);
        assert_eq!(output.unwrap(), Some(NaiveConsensusOutput::HighlightRed));
        assert_eq!(machine.state(), &NaiveConsensusState::Red);
    }
}
