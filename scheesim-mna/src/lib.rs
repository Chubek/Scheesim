use scheesim_lexparse::*;

pub struct Resistor {
    nonlinear: bool,
    resistance: f64,
}

impl Resistor {
    pub fn from(lexeme_line: LexemeLine, line_number: usize) -> Self {
        let mut nonlinear = false;
        let mut resistance = 0.0f64;

        for ll in lexeme_line {
            match ll {
                Lexeme::Arg(arg) => match arg {
                    Argument::Nonlinear => nonlinear = true,
                    Argument::Resistance(unit) => resistance = unit.get_corresponding_value(),
                    _ => error_out!("Wrong argument given to resistor in line {}, optional: -nonlinear, required: -resistance", line_number)
                },
                _ => error_out!("Wrong lexeme found in line {} for resistor. You can only pass arguments here.", line_number)
            }
        }
        
        if resistance == 0.0f64 {
            error_out!("You either did not pass -resistance in line {} or you passed a resistance of 0.0, it's required that you revise.", line_number);
        }


        Self { nonlinear, resistance }
    }
}

pub struct Capacitor {
    nonlinear: bool,
    dynamic: bool,
    capacitance: f64,
}

impl Capacitor {
    pub fn from(lexeme_line: LexemeLine, line_number: usize) -> Self {
        let mut nonlinear = false;
        let mut dynamic = false;
        let mut capacitance = 0.0f64;

        for ll in lexeme_line {
            match ll {
                Lexeme::Arg(arg) => match arg {
                    Argument::Nonlinear => nonlinear = true,
                    Argument::Dynamic => dynamic = true,
                    Argument::Capacitance(unit) => capacitance = unit.get_corresponding_value(),
                    _ => error_out!("Wrong argument given to capacitor in line {}, optional: -dynamic -nonlinear, required: -capacitance", line_number)
                },
                _ => error_out!("Wrong lexeme found in line {} for capacitor. You can only pass arguments here.", line_number)
            }
        }
        
        if capacitance == 0.0f64 {
            error_out!("You either did not pass -capacitance in line {} or you passed a capacitance of 0.0, it's required that you revise.", line_number);
        }


        Self { nonlinear, dynamic, capacitance }
    }
}


pub struct Inductor {
    nonlinear: bool,
    dynamic: bool,
    inductance: f64,
}

impl Inductor {
    pub fn from(lexeme_line: LexemeLine, line_number: usize) -> Self {
        let mut nonlinear = false;
        let mut dynamic = false;
        let mut inductance = 0.0f64;

        for ll in lexeme_line {
            match ll {
                Lexeme::Arg(arg) => match arg {
                    Argument::Nonlinear => nonlinear = true,
                    Argument::Dynamic => dynamic = true,
                    Argument::Inductance(unit) => inductance = unit.get_corresponding_value(),
                    _ => error_out!("Wrong argument given to capacitor in line {}, optional: -dynamic -nonlinear, required: -inductance", line_number)
                },
                _ => error_out!("Wrong lexeme found in line {} for capacitor. You can only pass arguments here.", line_number),
            }
        }
        
        if inductance == 0.0f64 {
            error_out!("You either did not pass -inductance in line {} or you passed an inductance of 0.0, it's required that you revise.", line_number);
        }


        Self { nonlinear, dynamic, inductance }
    }
}



pub enum TransistorType {
    BJT,
    MOSFET,
}

pub struct Transistor {
    power: f64,
    voltage: f64,
    junction_channel: JunctionChannel,
    trantype: TransistorType,
}

impl Transistor {
    pub fn from(lexeme_line: LexemeLine, line_number: usize) -> Self {
        let mut power = 0.0f64;
        let mut voltage = 0.0f64;
        let mut junction_channel = JunctionChannel::NPN;
        let mut trantype = TransistorType::BJT;

        for ll in lexeme_line {
            match ll {
                Lexeme::Arg(arg) => match arg {
                    Argument::Power(unit) => power = unit.get_corresponding_value(),
                    Argument::Voltage(currentage) => match currentage {
                        Currentage::Solo(unit) => voltage = unit.get_corresponding_value(),
                        _ => error_out!("Line {}, you may not use dependent/controlling voltage markers for transistor voltage", line_number),
                    },
                    Argument::JunctionChannel(jc) => {
                        match jc {
                            JunctionChannel::NPN | JunctionChannel::PNP => trantype = TransistorType::BJT,
                            JunctionChannel::N | JunctionChannel::P => trantype = TransistorType::MOSFET,
                            _ => error_out!("You may not use a junction type reserved for diodes for transistors, in line {}", line_number),
                        }

                        
                        junction_channel = jc;                        
                    },
                    _ => error_out!("Transistor in line {} got wrong type of argument", line_number),
                },
                _ => error_out!("Wrong lexeme found in line {} for transistor. You can only pass arguments here.", line_number),
            }
        }

        if power == 0.0f64 {
            error_out!("You either did not pass -power in line {} or you passed a power of 0.0, it's required that you revise.", line_number);
        }


        if voltage == 0.0f64 {
            error_out!("You either did not pass -voltage in line {} or you passed a voltage of 0.0, it's required that you revise.", line_number);
        }


        Self { power, voltage, junction_channel, trantype }
    }
}

pub struct Diode {
    power: f64,
    voltage: f64,
    junction: JunctionChannel,
}

impl Diode {
    pub fn from(lexeme_line: LexemeLine, line_number: usize) -> Self {
        let mut power = 0.0f64;
        let mut voltage = 0.0f64;
        let mut junction = JunctionChannel::NPN;

        for ll in lexeme_line {
            match ll {
                Lexeme::Arg(arg) => match arg {
                    Argument::Power(unit) => power = unit.get_corresponding_value(),
                    Argument::Voltage(currentage) => match currentage {
                        Currentage::Solo(unit) => voltage = unit.get_corresponding_value(),
                        _ => error_out!("Line {}, you may not use dependent/controlling voltage markers for transistor voltage", line_number),
                    },
                    Argument::JunctionChannel(jc) => match jc {
                            JunctionChannel::PN | JunctionChannel::NP => junction = jc,
                            _ => error_out!("You may not use a junction type reserved for transistors for a diode, in line {}", line_number),
                    },
                    _ => error_out!("Diode in line {} got wrong type of argument", line_number),
                },
                _ => error_out!("Wrong lexeme found in line {} for transistor. You can only pass arguments here.", line_number),
            }
        }

        if power == 0.0f64 {
            error_out!("You either did not pass -power in line {} or you passed a power of 0.0, it's required that you revise.", line_number);
        }


        if voltage == 0.0f64 {
            error_out!("You either did not pass -voltage in line {} or you passed a voltage of 0.0, it's required that you revise.", line_number);
        }


        Self { power, voltage, junction }
    }
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


pub enum Profile {
    Named(String),
    Default,
}


pub struct  NodeProfile {
    profile: Profile,
    components: Vec<Component>,
}

pub struct Node {
    number: u32,
    node_profiles: Vec<NodeProfile>,
    ins: Vec<Connection>,
    outs: Vec<Connection>,
    bases: Option<Vec<Connection>>

}