use colored::Colorize;
use std::collections::HashMap;
use std::process::exit;

const NODE_SEP_TOKEN_L1: char = '+';
const NAME_SEP_TOKEN_L1: char = '%';
const COMP_SEP_TOKEN_L1: char = '-';
const NAME_MARK_TOKEN_L2: char = '*';
const NODE_MARK_TOKEN_L2: char = '^';

pub enum EqualProperty {
    Bias(String, usize),
    Type(String, usize),
    Resistance(String, usize),
    Conductance(String, usize),
    Capacitance(String, usize),
    Inductance(String, usize),
    Voltage(String, usize),
    Current(String, usize),
    Power(String, usize),
    Frequency(String, usize),
}

impl EqualProperty {
    pub fn from(s: &str, line_num: usize) -> Self {
        let (key, value) = s.split_once("=").expect(
            format!(
                "{} at Line `{line_num}`: Error with key-value equal property: {s} ",
                "Error".red().bold()
            )
            .as_str(),
        );

        match key.trim().to_lowercase().as_str() {
            "bias" => Self::Bias(value.trim().to_string(), line_num),
            "type" => Self::Type(value.trim().to_string(), line_num),
            "resistance" => Self::Resistance(value.trim().to_string(), line_num),
            "conductance" => Self::Conductance(value.trim().to_string(), line_num),
            "capacitance" => Self::Capacitance(value.trim().to_string(), line_num),
            "inductance" => Self::Inductance(value.trim().to_string(), line_num),
            "voltage" => Self::Voltage(value.trim().to_string(), line_num),
            "current" => Self::Current(value.trim().to_string(), line_num),
            "power" => Self::Power(value.trim().to_string(), line_num),
            "frequency" => Self::Frequency(value.trim().to_string(), line_num),
            _ => {
                println!(
                    "{} at Line `{line_num}: Unknown quantity, consult documentation for allowed units`",
                    "Error".red().bold()
                );
                exit(1);
            }
        }
    }

    pub fn to_unit(&self) -> Option<Unit> {
        match self {
            EqualProperty::Bias(_, _) => None,
            EqualProperty::Type(_, _) => None,
            EqualProperty::Resistance(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Conductance(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Capacitance(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Inductance(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Voltage(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Current(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Power(value, line_num) => Some(Unit::from(value, *line_num)),
            EqualProperty::Frequency(value, line_num) => Some(Unit::from(value, *line_num)),
        }
    }

    pub fn to_bias(&self) -> Option<Bias> {
        match self {
            EqualProperty::Bias(value, line_num) => Some(Bias::from(value, *line_num)),
            _ => None,
        }
    }

    pub fn to_type(&self) -> Option<ElementType> {
        match self {
            EqualProperty::Type(value, line_num) => Some(ElementType::from(value, *line_num)),
            _ => None,
        }
    }

    pub fn get_key(&self) -> &str {
        match self {
            EqualProperty::Bias(_, _) => "bias",
            EqualProperty::Type(_, _) => "type",
            EqualProperty::Resistance(_, _) => "resistance",
            EqualProperty::Conductance(_, _) => "conductance",
            EqualProperty::Capacitance(_, _) => "capacitance",
            EqualProperty::Inductance(_, _) => "inductance",
            EqualProperty::Voltage(_, _) => "voltage",
            EqualProperty::Current(_, _) => "current",
            EqualProperty::Power(_, _) => "power",
            EqualProperty::Frequency(_, _) => "frequency",
        }
    }
}

pub trait FilterList<T, U> {
    fn filter_for(&self, key: T) -> Option<&U>;
    fn filter_for_mult(&self, key: T) -> Vec<&U>;
}

impl FilterList<&'static str, EqualProperty> for Vec<EqualProperty> {
    fn filter_for(&self, key: &str) -> Option<&EqualProperty> {
        self.iter().filter(|itm| itm.get_key() == key).next()
    }

    fn filter_for_mult(&self, key: &'static str) -> Vec<&EqualProperty> {
        self.iter().filter(|itm| itm.get_key() == key).collect()
    }
}

#[derive(Debug)]
pub enum Bias {
    NPN,
    PNP,
    Negative,
    Positive,
    NP,
    PN,
}

impl Bias {
    pub fn from(s: &str, line_num: usize) -> Self {
        match s.to_lowercase().as_str() {
            "npn" => Self::NPN,
            "pnp" => Self::PNP,
            "pos" | "positive" => Self::Positive,
            "neg" | "negative" => Self::Negative,
            "np" => Self::NP,
            "pn" => Self::PN,
            _ => {
                println!(
                    "{} at Line `{line_num}`: Wrong bias type: {s}",
                    "Error".red().bold()
                );
                exit(1);
            }
        }
    }
}

#[derive(Debug)]
pub struct MTETerminalPair {
    connector: String,
    connectee: String,
}

impl MTETerminalPair {
    pub fn from(connector: &String, connectee: &String) -> Self {
        Self {
            connector: connector.clone(),
            connectee: connectee.clone(),
        }
    }
}

#[derive(Debug)]
pub enum Component {
    Resistor(Unit, ElementType),
    Conductor(Unit, ElementType),
    Capacitor(Unit, ElementType),
    Inductor(Unit, ElementType),
    AcSource(Unit),
    DcSource(Option<Unit>, Option<Unit>, ElementType),
    BJT(Unit, Bias, ElementType),
    FET(Unit, Bias, ElementType),
    MOSFET(Unit, Bias, ElementType),
    Diode(Unit, Bias, ElementType),
}

impl Component {
    fn get_function_and_properties(s_in: &str, line_num: usize) -> (String, String) {
        let s = s_in.trim();

        let (func, prop) = s.split_once("<").expect(format!("{} at Line `{line_num}`: Error with element input, must be in form ElementComponent<properties_name=properties>", "Error".red().bold()).as_str());
        let prop_sans_right_angle = prop.replace(">", "");

        (func.to_string(), prop_sans_right_angle)
    }

    fn parse_equal_props(s: &str, line_num: usize) -> Vec<EqualProperty> {
        s.split(",")
            .map(|ss| EqualProperty::from(&ss.trim(), line_num))
            .collect()
    }

    pub fn from(s: &str, line_num: usize) -> Self {
        let (func, props) = Self::get_function_and_properties(s, line_num);
        let props_parsed = Self::parse_equal_props(&props, line_num);

        match func.trim().to_lowercase().as_str() {
            "resistor" => {
                let unit_opt = props_parsed.filter_for("resistance");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_ohm() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not ohm",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for resistor",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::Resistor(unit, ty)
            }
            "conductor" => {
                let unit_opt = props_parsed.filter_for("conductance");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_mho() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not mho/siemens",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for conductor",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::Conductor(unit, ty)
            }
            "capacitor" => {
                let unit_opt = props_parsed.filter_for("capacitance");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_farad() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not farad",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for capacitor",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::Capacitor(unit, ty)
            }
            "inductor" => {
                let unit_opt = props_parsed.filter_for("inductance");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_henry() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not henry",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for inductor",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::Inductor(unit, ty)
            }
            "acsource" => {
                let unit_opt = props_parsed.filter_for("frequency");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_hertz() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not hertz",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for sine/ac source",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                Self::AcSource(unit)
            }
            "dcsource" => {
                let voltages = props_parsed.filter_for_mult("voltage");
                let currents = props_parsed.filter_for_mult("currents");
                let ty_opt = props_parsed.filter_for("type");

                let (unit_1, unit_2) = if voltages.len() == 1 && currents.len() == 0 {
                    let solo_voltage = voltages[0].to_unit();

                    let (u1, u2) = match solo_voltage {
                        Some(v) => match v.voltage_solos() {
                            true => (Some(v), None),
                            false => {
                                println!("{} at Line `{line_num}`: Single voltage must be <solo> of Volt unit", "Error".red().bold());
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: Voltage/Current undefined",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    (u1, u2)
                } else if voltages.len() == 0 && currents.len() == 1 {
                    let solo_current = currents[0].to_unit();

                    let (u1, u2) = match solo_current {
                        Some(i) => match i.current_solos() {
                            true => (None, Some(i)),
                            false => {
                                println!("{} at Line `{line_num}`: Single current must be <solo> of Amper unit", "Error".red().bold());
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: Voltage/Current undefined",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    (u1, u2)
                } else if voltages.len() == 2 && currents.len() == 0 {
                    let mut iter = voltages.iter();

                    let (voltage_1, voltage_2) = (iter.next().unwrap(), iter.next().unwrap());
                    let (voltage_unit_1, voltage_unit_2) =
                        (voltage_1.to_unit(), voltage_2.to_unit());

                    if voltage_unit_1.is_none() && voltage_unit_2.is_none() {
                        println!(
                            "{} at Line `{line_num}`: No voltages exist",
                            "Error".red().bold()
                        );
                        exit(1);
                    }

                    let (v1, v2) = (voltage_unit_1.unwrap(), voltage_unit_2.unwrap());

                    if !(v1.is_volts() && v2.is_amps()) {
                        println!(
                            "{} at Line `{line_num}`: Two voltages must have unit Volt",
                            "Error".red().bold()
                        );
                        exit(1);
                    }

                    let (v1u, v2u) = if (v1.voltage_controls() && v2.voltage_depends())
                        || (v1.voltage_depends() && v2.voltage_controls())
                    {
                        match v1.voltage_controls() {
                            true => (Some(v1), Some(v2)),
                            false => (Some(v2), Some(v1)),
                        }
                    } else {
                        println!("{} at Line `{line_num}`: When two voltages, one must be controller, and the other dependent", "Error".red().bold());
                        exit(1);
                    };

                    (v1u, v2u)
                } else if voltages.len() == 0 && currents.len() == 2 {
                    let mut iter = currents.iter();

                    let (current_1, current_2) = (iter.next().unwrap(), iter.next().unwrap());
                    let (current_unit_1, current_unit_2) =
                        (current_1.to_unit(), current_2.to_unit());

                    if current_unit_1.is_none() && current_unit_2.is_none() {
                        println!(
                            "{} at Line `{line_num}`: No currents exist",
                            "Error".red().bold()
                        );
                    }

                    let (i1, i2) = (current_unit_1.unwrap(), current_unit_2.unwrap());

                    if !(i1.is_amps() && i2.is_amps()) {
                        println!(
                            "{} at Line `{line_num}`: Two currents must be of unit Amps",
                            "Error".red().bold()
                        );
                        exit(1);
                    }

                    let (i1u, i2u) = if (i1.current_controls() && i2.current_depends())
                        || (i1.current_depends() && i2.current_controls())
                    {
                        match i1.current_controls() {
                            true => (Some(i1), Some(i2)),
                            false => (Some(i2), Some(i1)),
                        }
                    } else {
                        println!("{} at Line `{line_num}`: When two currents, one must be controller, and the other dependent", "Error".red().bold());
                        exit(1);
                    };

                    (i1u, i2u)
                } else if voltages.len() == 1 && currents.len() == 1 {
                    let (v, i) = (
                        voltages.iter().next().unwrap(),
                        currents.iter().next().unwrap(),
                    );

                    let (vu, iu) = (v.to_unit().unwrap(), i.to_unit().unwrap());

                    if !(vu.is_volts() && iu.is_amps()) {
                        println!("{} at Line `{line_num}`:  Voltage must be of unit volts and current must be of unit amps", "Error".red().bold());
                        exit(1);
                    }

                    let (u1, u2) = match vu.voltage_controls() {
                        true => match iu.current_depends() {
                            true => (Some(vu), Some(iu)),
                            false => {
                                println!("{} at Line `{line_num}`: If voltage controls, current must depent", "Error".red().bold());
                                exit(1);
                            }
                        },
                        false => match iu.current_controls() {
                            true => (Some(iu), Some(vu)),
                            false => {
                                println!("{} at Line `{line_num}`: Either voltage, or current, should control and ther other must be dependent", "Error".red().bold());
                                exit(1);
                            }
                        },
                    };

                    (u1, u2)
                } else {
                    println!("{} at Line `{line_num}`: Wrong voltage/current units, must have one solo voltage/current, or two controller/dependent voltage/current or two controller/dependent current/voltage", "Error".red().bold());
                    exit(1);
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::DcSource(unit_1, unit_2, ty)
            }
            "bjt" | "bipolarjunction" | "bipolarjunctiontransistor" => {
                let unit_opt = props_parsed.filter_for("power");
                let bias_opt = props_parsed.filter_for("bias");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_watts() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not watts",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for BJT",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let bias = match bias_opt {
                    Some(b) => match b.to_bias() {
                        Some(bias) => bias,
                        None => {
                            println!(
                                "{} at Line `{line_num}`: Error getting bias BJT",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    },
                    None => {
                        println!(
                            "{} at Line `{line_num}`: Bias not specified for BJT",
                            "Error".red().bold()
                        );
                        exit(1);
                    }
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::BJT(unit, bias, ty)
            }
            "fet" | "fieldeffect" | "fieldeffecttransistor" => {
                let unit_opt = props_parsed.filter_for("power");
                let bias_opt = props_parsed.filter_for("bias");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_watts() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not watts",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for FET",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let bias = match bias_opt {
                    Some(b) => match b.to_bias() {
                        Some(bias) => bias,
                        None => {
                            println!(
                                "{} at Line `{line_num}`: Error getting bias FET",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    },
                    None => {
                        println!(
                            "{} at Line `{line_num}`: Bias not specified for FET",
                            "Error".red().bold()
                        );
                        exit(1);
                    }
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::FET(unit, bias, ty)
            }
            "mosfet" | "mosfieldeffect" | "mosfieldeffecttransistor" => {
                let unit_opt = props_parsed.filter_for("power");
                let bias_opt = props_parsed.filter_for("bias");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_watts() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not watts",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for MOSFET",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let bias = match bias_opt {
                    Some(b) => match b.to_bias() {
                        Some(bias) => bias,
                        None => {
                            println!(
                                "{} at Line `{line_num}`: Error getting bias MOSFET",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    },
                    None => {
                        println!(
                            "{} at Line `{line_num}`: Bias not specified for MOSFET",
                            "Error".red().bold()
                        );
                        exit(1);
                    }
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::BJT(unit, bias, ty)
            }
            "diode" => {
                let unit_opt = props_parsed.filter_for("power");
                let bias_opt = props_parsed.filter_for("bias");
                let ty_opt = props_parsed.filter_for("type");

                let unit = if unit_opt.is_some() {
                    let unit_prop = unit_opt.unwrap();
                    let unit = unit_prop.to_unit();

                    let u = match unit {
                        Some(u) => match u.is_watts() {
                            true => u,
                            false => {
                                println!(
                                    "{} at Line `{line_num}`: Unit is not watts",
                                    "Error".red().bold()
                                );
                                exit(1);
                            }
                        },
                        None => {
                            println!(
                                "{} at Line `{line_num}`: No unit found",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    };

                    u
                } else {
                    println!(
                        "{} at Line `{line_num}`: No unit specified for diode",
                        "Error".red().bold()
                    );
                    exit(1);
                };

                let bias = match bias_opt {
                    Some(b) => match b.to_bias() {
                        Some(bias) => bias,
                        None => {
                            println!(
                                "{} at Line `{line_num}`: Error getting bias diode",
                                "Error".red().bold()
                            );
                            exit(1);
                        }
                    },
                    None => {
                        println!(
                            "{} at Line `{line_num}`: Bias not specified for diode",
                            "Error".red().bold()
                        );
                        exit(1);
                    }
                };

                let ty = match ty_opt {
                    Some(eq) => match eq.to_type() {
                        Some(t) => t,
                        None => ElementType::Linear,
                    },
                    None => ElementType::Linear,
                };

                Self::BJT(unit, bias, ty)
            }
            _ => {
                println!("{} at Line `{line_num}`: Unknown element, consult the documentation for a full list of allowed element and element names", "Error".red().bold());
                exit(1);
            }
        }
    }

    pub fn get_type(&self) -> &str {
        match self {
            Component::Resistor(_, _) => "resistor",
            Component::Conductor(_, _) => "conductor",
            Component::Capacitor(_, _) => "capacitor",
            Component::Inductor(_, _) => "inductor",
            Component::AcSource(_) => "a.c.source",
            Component::DcSource(_, _, _) => "d.c.source",
            Component::BJT(_, _, _) => "bjt",
            Component::FET(_, _, _) => "fet",
            Component::MOSFET(_, _, _) => "mosfet",
            Component::Diode(_, _, _) => "diode",
        }
    }

    pub fn is_type(&self, supposed_type: &str) -> bool {
        let ty = self.get_type();

        ty == supposed_type
    }
}

#[derive(Debug)]
pub enum ElementType {
    Linear,
    NonLinear,
    Dynamic,
}

impl ElementType {
    pub fn from(value: &str, line_num: usize) -> Self {
        match value.to_lowercase().as_str() {
            "linear" => Self::Linear,
            "nonlinear" => Self::NonLinear,
            "dynamic" => Self::Dynamic,
            _ => {
                println!(
                    "{} at Line `{line_num}`: Wrong element type given: {value}",
                    "Error".red().bold()
                );
                exit(1);
            }
        }
    }
}

#[derive(Debug)]
pub struct Element {
    name: String,
    component: Option<Component>,
    terminal: ElementTerminal,
    subnodes: Option<Vec<Node>>,
}

impl Element {
    pub fn from(s_in: &str, line_num: usize) -> Self {
        let s = s_in.trim();
        let (s_terminal, s_component) = s.split_once(COMP_SEP_TOKEN_L1).expect(&format!("{} Name/Terminal specifications and component names must be seperated by {NAME_SEP_TOKEN_L1}", "Error".red().bold()));

        let (name, terminal) = Self::parse_terminal(&s_terminal, line_num);

        match name.chars().next().unwrap() == NAME_MARK_TOKEN_L2 {
            true => {
                let name = name.replace("*", "");
                let subnodes: Vec<Node> = s_component
                    .split(NODE_MARK_TOKEN_L2)
                    .map(|s| s.trim())
                    .map(|s| Node::from(s, line_num))
                    .collect();

                Self {
                    name,
                    component: None,
                    terminal,
                    subnodes: Some(subnodes),
                }
            }
            false => {
                let component = Component::from(&s_component, line_num);

                Self {
                    name,
                    component: Some(component),
                    terminal,
                    subnodes: None,
                }
            }
        }
    }

    fn parse_terminal(s_terminal_in: &str, line_num: usize) -> (String, ElementTerminal) {
        let s_terminal = s_terminal_in.trim();

        let (name, s) = s_terminal.split_once("<").expect(
            format!(
                "{} at Line `{line_num}`: Element name should be in style name<MT/BT/TT[*connections]>",
                "Error".red().bold()
            )
            .as_str(),
        );

        let conns = ElementTerminal::from(s, line_num);

        (name.trim().to_string(), conns)
    }
}

#[derive(Debug)]
pub enum TerminalConnection {
    Named(String),
    External,
    Ground,
    InternalGround,
}

impl TerminalConnection {
    pub fn from(s: &str, line_num: usize) -> Self {
        match s.to_lowercase().as_str() {
            "ext" | "external" | "extrn" | "extern" => Self::External,
            "grnd" | "ground" | "grd" | "gr" => Self::Ground,
            "igrnd" | "iground" | "igrd" | "igr" | "internalgrnd" | "internalground" => {
                Self::InternalGround
            }
            _ => match s.contains(">") | s.contains("<") | s.contains(";") {
                true => {
                    println!("{} at Line `{line_num}`: Connection name `{s}` contains illegal character (>, < or ;)", "Error".red().bold());
                    exit(1);
                }
                false => Self::Named(s.to_lowercase()),
            },
        }
    }
}

pub trait FromEqualSeperatedSeq {
    fn from_eq_sep_conn(&mut self, v: &Vec<&str>, line_num: usize);
}

impl FromEqualSeperatedSeq for HashMap<String, TerminalConnection> {
    fn from_eq_sep_conn(&mut self, v: &Vec<&str>, line_num: usize) {
        v.into_iter().for_each(|s| {
            if !s.contains("=") {
                println!(
                    "{} Item {s} must be equal-sign-seperated!",
                    "Error".red().bold()
                );
                exit(1);
            }

            let (key, value) = s.split_once("=").expect(
                format!(
                    "{} Error splitting `{s}` by equal sign",
                    "Error".red().bold()
                )
                .as_str(),
            );

            self.insert(key.to_string(), TerminalConnection::from(value, line_num));
        });
    }
}

#[derive(Debug)]
pub enum ElementTerminal {
    BiTerminal(TerminalConnection, TerminalConnection),
    TriTerminal(TerminalConnection, TerminalConnection, TerminalConnection),
    MultiTerminal(HashMap<String, TerminalConnection>),
}

#[derive(Debug)]
pub struct NodeTerminal {
    conns_in: Option<Vec<TerminalConnection>>,
    conns_out: Option<Vec<TerminalConnection>>,
}

impl NodeTerminal {
    pub fn from(s_in: &str, line_num: usize) -> Self {
        let s = s_in.trim();

        let replaced_double_right_angle = s.replace(">>", "");

        if !(replaced_double_right_angle.to_lowercase().contains("in")
            && replaced_double_right_angle.contains("out"))
        {
            println!(
                "{} at Line `{line_num}`: Node connections must have either in, out or both",
                "Error".red().bold()
            );
            exit(1);
        }

        let mut split = replaced_double_right_angle.split(";");

        enum InOut {
            In(Vec<TerminalConnection>),
            Out(Vec<TerminalConnection>),
        }

        impl InOut {
            pub fn is_in(&self) -> bool {
                match self {
                    Self::In(_) => true,
                    Self::Out(_) => false,
                }
            }

            pub fn unwrap(self) -> Vec<TerminalConnection> {
                match self {
                    Self::In(v) => v,
                    Self::Out(v) => v,
                }
            }
        }

        let (conns_in, conns_out) = match split.next() {
            Some(s) => {
                let conns = Self::parse_within_parans(s, line_num);

                let first = match s.to_lowercase().contains("in") {
                    true => InOut::In(conns),
                    false => InOut::Out(conns),
                };

                let second = match split.next() {
                    Some(s) => {
                        let conns = Self::parse_within_parans(s, line_num);

                        match s.to_lowercase().contains("in") {
                            true => Some(InOut::In(conns)),
                            false => Some(InOut::Out(conns)),
                        }
                    }
                    None => None,
                };

                let (fin, fout) = match second {
                    Some(inout) => {
                        if (inout.is_in() && first.is_in()) || (!inout.is_in() && !first.is_in()) {
                            println!(
                                "{} at Line `{line_num}`: Samewise ins/outs",
                                "Error".red().bold()
                            );
                            exit(1);
                        }

                        if first.is_in() {
                            (Some(first.unwrap()), Some(inout.unwrap()))
                        } else {
                            (Some(inout.unwrap()), Some(first.unwrap()))
                        }
                    }
                    None => {
                        if first.is_in() {
                            (Some(first.unwrap()), None)
                        } else {
                            (None, Some(first.unwrap()))
                        }
                    }
                };

                (fin, fout)
            }
            None => {
                println!(
                    "{} at Line `{line_num}`: At least one in or one out is required!",
                    "Error".red().bold()
                );
                exit(1);
            }
        };

        Self {
            conns_in,
            conns_out,
        }
    }

    fn parse_within_parans(s: &str, line_num: usize) -> Vec<TerminalConnection> {
        s.replace("in", "")
            .replace("out", "")
            .replace("=", "")
            .replace("(", "")
            .replace(")", "")
            .split(",")
            .map(|ss| ss.trim())
            .map(|ss| TerminalConnection::from(ss, line_num))
            .collect()
    }
}

impl ElementTerminal {
    pub fn from(s_in: &str, line_num: usize) -> Self {
        if !s_in.contains("]>:") || s_in.contains("]>>:") {
            println!("{} at Line `{line_num}`: Wrong pattern for compnent name, please consult documentation for allowed patterns for component names", "Error".red().bold());
            exit(1);
        }

        let rem_right_square = s_in.replace("]>:", "");
        let (ty, conns) = rem_right_square.split_once("[").expect(
            format!(
                "{} at Line `{line_num}`: Terminal listing must be in form <type>[*conns]",
                "Error".red().bold()
            )
            .as_str(),
        );

        let conns_split = conns.split(";").map(|sp| sp.trim()).collect::<Vec<_>>();

        match ty.to_lowercase().as_str() {
            "bt" | "biterminal" | "biterm" => {
                let (conn1, conn2) = match conns_split.len() {
                    2 => {
                        let mut iter = conns_split.into_iter();
                        let (conn_str1, conn_str2) = (iter.next().unwrap(), iter.next().unwrap());
                        let (conn1, conn2) = (
                            TerminalConnection::from(conn_str1, line_num),
                            TerminalConnection::from(conn_str2, line_num),
                        );
                        (conn1, conn2)
                    }
                    _ => {
                        println!(
                            "{} at Line `{line_num}`: BiTerminal connection needs exactly 2 connections", "Error".red().bold()
                        );
                        exit(1);
                    }
                };

                Self::BiTerminal(conn1, conn2)
            }
            "tt" | "triterminal" | "triterm" => {
                let (conn1, conn2, conn3) = match conns_split.len() {
                    3 => {
                        let mut iter = conns_split.into_iter();
                        let (conn_str1, conn_str2, conn_str3) = (
                            iter.next().unwrap(),
                            iter.next().unwrap(),
                            iter.next().unwrap(),
                        );
                        let (conn1, conn2, conn3) = (
                            TerminalConnection::from(conn_str1, line_num),
                            TerminalConnection::from(conn_str2, line_num),
                            TerminalConnection::from(conn_str3, line_num),
                        );

                        (conn1, conn2, conn3)
                    }
                    _ => {
                        println!(
                            "{} at Line `{line_num}`: TriTerminal connection needs exactly 3 connections", "Error".red().bold()
                        );
                        exit(1);
                    }
                };

                Self::TriTerminal(conn1, conn2, conn3)
            }
            "mt" | "multiterminal" | "multiterm" => match conns_split.len() != 0 {
                true => {
                    let mut hm = HashMap::<String, TerminalConnection>::new();
                    hm.from_eq_sep_conn(&conns_split, line_num);

                    Self::MultiTerminal(hm)
                }
                false => {
                    println!("{} at Line `{line_num}`: MultiTerminal connections must be expressed within the component", "Error".red().bold());
                    exit(1);
                }
            },
            _ => {
                println!("{} at Line `{line_num}`: Wrong terminal connection type, please consult documentation for a list of allowed terminal connection types", "Error".red().bold());
                exit(1);
            }
        }
    }

    pub fn is_multiterminal(&self) -> bool {
        match self {
            Self::MultiTerminal(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    name: String,
    terminal: NodeTerminal,
    elements: Vec<Element>,
}

impl Node {
    pub fn from(s_in: &str, line_num: usize) -> Self {
        let s = s_in.trim();

        let (node_name_s, components_and_nodes_s) = s.split_once(">>").expect(&format!("{} at Line `{line_num}`: Wrong pattern for nodes and component list, please consult documentation", "Error".red().bold()));
        let (name, node_s) = node_name_s.split_once("<<").expect(&format!("{} at Line `{line_num}`: Wrong pattern for node name, please consult documentation for allowed patterns", "Error".red().bold()));
        let terminal = NodeTerminal::from(node_s, line_num);

        let elements = components_and_nodes_s
            .split(NAME_SEP_TOKEN_L1)
            .filter(|s| s.len() > 0)
            .map(|s| s.trim())
            .enumerate()
            .map(|(i, s)| Element::from(&s, line_num + i + 1))
            .collect::<Vec<_>>();

        Self {
            terminal,
            elements,
            name: name.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum UnitMultiplier {
    Quetta,
    Ronna,
    Yotta,
    Zetta,
    Exa,
    Petta,
    Tera,
    Giga,
    Mega,
    Kilo,
    Hecto,
    Deka,
    Deci,
    Centi,
    Milli,
    Micro,
    Nano,
    Pico,
    Femto,
    Atto,
    Zepto,
    Yocto,
    Ronto,
    Quecto,
    NoMultiplier,
}

impl UnitMultiplier {
    pub fn from(s: &str) -> Self {
        match s {
            "Q" => Self::Quecto,
            "R" => Self::Ronna,
            "Y" => Self::Yotta,
            "Z" => Self::Zetta,
            "E" => Self::Exa,
            "P" => Self::Petta,
            "T" => Self::Tera,
            "G" => Self::Giga,
            "M" => Self::Mega,
            "k" => Self::Kilo,
            "h" => Self::Hecto,
            "da" => Self::Deka,
            "d" => Self::Deci,
            "c" => Self::Centi,
            "m" => Self::Milli,
            "u" => Self::Micro,
            "n" => Self::Nano,
            "p" => Self::Pico,
            "f" => Self::Femto,
            "a" => Self::Atto,
            "z" => Self::Zepto,
            "y" => Self::Yocto,
            "r" => Self::Ronto,
            "q" => Self::Quecto,
            _ => Self::NoMultiplier,
        }
    }

    pub fn multiply_unit(&self, unit: f64) -> f64 {
        match self {
            UnitMultiplier::Quetta => unit * 1e30,
            UnitMultiplier::Ronna => unit * 1e27,
            UnitMultiplier::Yotta => unit * 1e24,
            UnitMultiplier::Zetta => unit * 1e21,
            UnitMultiplier::Exa => unit * 1e18,
            UnitMultiplier::Petta => unit * 1e15,
            UnitMultiplier::Tera => unit * 1e12,
            UnitMultiplier::Giga => unit * 1e9,
            UnitMultiplier::Mega => unit * 1e6,
            UnitMultiplier::Kilo => unit * 1e3,
            UnitMultiplier::Hecto => unit * 1e2,
            UnitMultiplier::Deka => unit * 1e1,
            UnitMultiplier::Deci => unit * 1e-1,
            UnitMultiplier::Centi => unit * 1e-2,
            UnitMultiplier::Milli => unit * 1e-3,
            UnitMultiplier::Micro => unit * 1e-6,
            UnitMultiplier::Nano => unit * 1e-9,
            UnitMultiplier::Pico => unit * 1e-12,
            UnitMultiplier::Femto => unit * 1e-15,
            UnitMultiplier::Atto => unit * 1e-18,
            UnitMultiplier::Zepto => unit * 1e-21,
            UnitMultiplier::Yocto => unit * 1e-24,
            UnitMultiplier::Ronto => unit * 1e-27,
            UnitMultiplier::Quecto => unit * 1e-30,
            UnitMultiplier::NoMultiplier => unit,
        }
    }
}

#[derive(Debug)]
pub enum ControlStat {
    Controller,
    Dependent,
    Solo,
}

impl ControlStat {
    pub fn is_controller(&self) -> bool {
        match self {
            Self::Controller => true,
            _ => false,
        }
    }

    pub fn is_dependent(&self) -> bool {
        match self {
            Self::Dependent => true,
            _ => false,
        }
    }

    pub fn is_solo(&self) -> bool {
        match self {
            Self::Solo => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum Unit {
    Henry(f64, UnitMultiplier),
    Farad(f64, UnitMultiplier),
    Ohm(f64, UnitMultiplier),
    Mho(f64, UnitMultiplier),
    Hertz(f64, UnitMultiplier),
    Amps(f64, UnitMultiplier, ControlStat),
    Volts(f64, UnitMultiplier, ControlStat),
    Watts(f64, UnitMultiplier),
}

impl Unit {
    fn seperate_num_from_unit(s: &str, line_num: usize) -> (String, String, String) {
        let mut quantitiy = String::new();
        let mut multiplier = String::new();
        let mut unit = String::new();

        for ch in s.chars() {
            if ch.is_numeric() || ch == '.' {
                quantitiy.push(ch);
            } else if ch.is_alphabetic() {
                if vec![
                    'q', 'r', 'y', 'z', 'a', 'f', 'p', 'n', 'u', 'm', 'c', 'd', 'h', 'k', 'M', 'G',
                    'T', 'P', 'T', 'E', 'Z', 'Y', 'R', 'Q',
                ]
                .into_iter()
                .any(|u| u == ch)
                {
                    match multiplier.len() == 0 {
                        true => multiplier.push(ch),
                        false => match ch == 'a' {
                            true => multiplier.push(ch),
                            false => continue,
                        },
                    }
                } else {
                    unit.push(ch);
                }
            } else {
                println!("{} at Line `{line_num}`: Illegal character in quantity, neither unit, multiplier, nor number (with or without decimal). Illegal character is: {}", "Error".red().bold(), ch);
                exit(1);
            }
        }

        (quantitiy, multiplier, unit)
    }

    pub fn from(s_in: &str, line_num: usize) -> Self {
        let s = s_in.trim();

        let (value_str, mult_str, unit_str) = Self::seperate_num_from_unit(&s, line_num);

        let value: f64 = value_str.parse().expect(
            format!(
                "{} at Line `{line_num}`: Error parsing value: {value_str}",
                "Error".red().bold()
            )
            .as_str(),
        );
        let multiplier = UnitMultiplier::from(&mult_str);

        match unit_str.to_lowercase().as_str() {
            "h" => Self::Henry(value, multiplier),
            "f" => Self::Farad(value, multiplier),
            "ohm" | "^" => Self::Ohm(value, multiplier),
            "mho" | "#" | "s" | "siemens" => Self::Mho(value, multiplier),
            "hertz" | "hz" => Self::Hertz(value, multiplier),
            "v" | "volts" | "volt" => Self::Volts(value, multiplier, ControlStat::Solo),
            "A" | "amps" | "amp" => Self::Amps(value, multiplier, ControlStat::Solo),
            "v@dep" | "volts@dep" | "volt@dep" => {
                Self::Volts(value, multiplier, ControlStat::Dependent)
            }
            "a@dep" | "amps@dep" | "amp@dep" => {
                Self::Amps(value, multiplier, ControlStat::Dependent)
            }
            "v@ctrl" | "volts@ctrl" | "volt@ctrl" => {
                Self::Volts(value, multiplier, ControlStat::Controller)
            }
            "a@ctrl" | "amps@ctrl" | "amp@ctrl" => {
                Self::Amps(value, multiplier, ControlStat::Controller)
            }
            "w" | "watt" | "watts" => Self::Watts(value, multiplier),
            _ => {
                println!(
                    "{} The given unit is not SI or wrongly written: {}. Please consult the documentation for an allowed list of units.",
                    "Error".red().bold(),
                    unit_str.bright_yellow().bold()
                );
                exit(1);
            }
        }
    }

    pub fn is_ohm(&self) -> bool {
        match self {
            Self::Ohm(_, _) => true,
            _ => false,
        }
    }

    pub fn is_henry(&self) -> bool {
        match self {
            Self::Henry(_, _) => true,
            _ => false,
        }
    }

    pub fn is_farad(&self) -> bool {
        match self {
            Self::Farad(_, _) => true,
            _ => false,
        }
    }

    pub fn is_mho(&self) -> bool {
        match self {
            Self::Mho(_, _) => true,
            _ => false,
        }
    }

    pub fn is_hertz(&self) -> bool {
        match self {
            Self::Hertz(_, _) => true,
            _ => false,
        }
    }

    pub fn is_amps(&self) -> bool {
        match self {
            Self::Amps(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_volts(&self) -> bool {
        match self {
            Self::Volts(_, _, _) => true,
            _ => false,
        }
    }

    pub fn is_watts(&self) -> bool {
        match self {
            Self::Watts(_, _) => true,
            _ => false,
        }
    }

    pub fn current_controls(&self) -> bool {
        match self {
            Self::Amps(_, _, b) => b.is_controller(),
            _ => false,
        }
    }

    pub fn voltage_controls(&self) -> bool {
        match self {
            Self::Volts(_, _, b) => b.is_controller(),
            _ => false,
        }
    }

    pub fn current_depends(&self) -> bool {
        match self {
            Self::Amps(_, _, b) => b.is_dependent(),
            _ => false,
        }
    }

    pub fn voltage_depends(&self) -> bool {
        match self {
            Self::Volts(_, _, b) => b.is_dependent(),
            _ => false,
        }
    }

    pub fn current_solos(&self) -> bool {
        match self {
            Self::Amps(_, _, b) => b.is_solo(),
            _ => false,
        }
    }

    pub fn voltage_solos(&self) -> bool {
        match self {
            Self::Volts(_, _, b) => b.is_solo(),
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct ScheesimSchema(Vec<Node>);

impl ScheesimSchema {
    pub fn from(s_in: &str) -> Self {
        let s_trimmed = s_in.trim().replace("\n", "");
        let s = Self::replace_non_ascii(&s_trimmed);

        let nodes = s
            .split(NODE_SEP_TOKEN_L1)
            .filter(|s| s.len() > 0)
            .enumerate()
            .map(|(line_num, s)| Node::from(s, line_num + 1))
            .collect::<Vec<_>>();

        Self(nodes)
    }

    fn replace_non_ascii(s: &str) -> String {
        s.chars()
            .map(|ch| match ch {
                'μ' => 'u',
                'Ω' => '^',
                'ʊ' => '#',
                _ => ch,
            })
            .collect::<String>()
    }
}
