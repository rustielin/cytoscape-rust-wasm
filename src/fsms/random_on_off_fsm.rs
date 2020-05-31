use rust_fsm::*;

state_machine! {
    pub RandomOnOff(Off)

    On => {
        Successful => Off [Dim],
        Unsuccessful => On,
    },
    Off => {
        Successful => On [Highlight],
        Unsuccessful => Off,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_on_off() {
        let mut machine: StateMachine<RandomOnOff> = StateMachine::new();
        let _ = machine.consume(&RandomOnOffInput::Successful);
        assert_eq!(machine.state(), &RandomOnOffState::On);

        let _ = machine.consume(&RandomOnOffInput::Successful);
        assert_eq!(machine.state(), &RandomOnOffState::Off);
    }
}
