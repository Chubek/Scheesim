use std::any::Any;

use scheesim_lexparse::*;

pub trait ComponentStamp<T: Sized + 'static>  {
    fn stamp(&self) -> T;
}

pub enum Profile {
    Named(String),
    Default,
}

pub struct Resistor {
    nonlinear: bool,
    resistance: f64,
    profile: Profile,
}

pub struct Capacitor {
    nonlinear: bool,
    dynamic: bool,
    capacitance: f64,
    profile: Profile,
}

pub struct Inductor {
    nonlinear: bool,
    dynamic: bool,
    inductance: f64,
    profile: Profile,
}

pub enum TransistorType {
    BJT,
    MOSFET,
}

pub struct Transistor {
    power: Option<f64>,
    voltage: Option<f64>,
    junction_channel: JunctionChannel,
}

pub struct Diode {
    power: Option<f64>,
    voltage: Option<f64>,
    junction_channel: JunctionChannel,
}

pub struct ACSweep {
    freq: f64,
    max_voltage: f64,
}

pub struct IndependentVoltageSource(f64);
pub struct IndependentCurrentSource(f64);

pub struct CurrentControlledCurrentSource {
    dom_current: f64,
    sub_current: f64,
}

pub struct CurrentControlledVoltageSource {
    dom_current: f64,
    sub_voltage: f64,
}

pub struct VoltageControlledCurrentSource {
    dom_voltage: f64,
    sub_current: f64,
}

pub struct VoltageControlledVoltageSource {
    dom_voltage: f64,
    sub_voltage: f64,
}


pub enum Component {
    Resistor(Resistor),
    Capacitor(Capacitor),
    Inductor(Inductor),
    Transistor(Transistor),
    Diode(Diode),
    ACSweep(ACSweep),
    IndependentCurrentSource(IndependentCurrentSource),
    IndependentVoltageSource(IndependentVoltageSource),
    CurrentControlledCurrentSource(CurrentControlledCurrentSource),
    CurrentControlledVoltageSource(CurrentControlledVoltageSource),
    VoltageControlledCurrentSource(VoltageControlledCurrentSource),
    VoltageControlledVoltageSource(VoltageControlledVoltageSource),
}

pub struct Node {
    number: u32,
    components: Vec<Component>,
    ins: Vec<Connection>,
    outs: Vec<Connection>,
    bases: Option<Vec<Connection>>


}