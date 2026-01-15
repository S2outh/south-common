use tmtc_system::*;

// # Telecommands
#[derive(TMValue)]
pub enum Telecommand {
    RocketLST(LSTCommand),
    EPS(EPSCommand),
}

#[derive(TMValue)]
pub enum LSTCommand {
    Reboot,
}

#[derive(TMValue)]
pub enum EPSCommand {
    SetSource(FlipFlopState, Option<u8>),
    EnableSink(Sink, Option<u8>),
    DisableSink(Sink, Option<u8>),
}

#[derive(TMValue, Clone, Copy)]
pub enum FlipFlopState {
    On,
    Bat1,
    Bat2,
    AuxPwr,
}

#[derive(TMValue, Clone, Copy)]
pub enum Sink {
    RocketLST,
    SensorUpper,
    GPS,
    RocketHD,
}
