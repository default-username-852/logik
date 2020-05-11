use crate::data::component::{Component, PortType, StateChange, Pokeable};
use crate::data::subnet::SubnetState;
use std::collections::HashMap;
use std::cell::Cell;
use crate::{map, port_or_default};
use intertrait::cast_to;

#[derive(Debug)]
pub(crate) struct Constant {
    #[cfg(not(test))]
    state: Cell<bool>,
    #[cfg(test)]
    pub state: Cell<bool>,
}

impl Component for Constant {
    fn ports(&self) -> usize {
        1
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, _: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let val = match self.state.get() {
            true => SubnetState::On,
            false => SubnetState::Off,
        };
        map!(0 => val)
    }
}

impl Constant {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}

#[derive(Debug)]
pub(crate) struct Button {
    #[cfg(not(test))]
    state: Cell<bool>,
    #[cfg(test)]
    pub state: Cell<bool>,
}

impl Component for Button {
    fn ports(&self) -> usize {
        1
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, _: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let val = match self.state.get() {
            true => SubnetState::On,
            false => SubnetState::Off,
        };
        map!(0 => val)
    }
}

#[cast_to]
impl Pokeable for Button {
    fn pokeable_areas(&self) -> i32 {
        1
    }
    
    fn poke_start(&mut self, area: i32) {
        if area == 0 {
            self.state.set(true);
        }
    }
    
    fn poke_end(&mut self, area: i32) {
        if area == 0 {
            self.state.set(false);
        }
    }
}

impl Button {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}

#[derive(Debug)]
pub(crate) struct DFlipFlop {
    #[cfg(not(test))]
    state: Cell<bool>,
    #[cfg(test)]
    pub state: Cell<bool>,
}

impl Component for DFlipFlop {
    fn ports(&self) -> usize {
        5 // 0 is D, 1 is clock, 2 is disable, 3 is Q, 4 is Q inverse
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0..=2 => Some(PortType::Input),
            3 | 4 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, data: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let d = port_or_default!(data, 0);
        let clock = data.get(&1).map(|e| e.rising()).unwrap_or(false);
        let disable = port_or_default!(data, 2);
        
        if clock && disable != SubnetState::On {
            if d == SubnetState::On {
                self.state.set(true);
            } else if d == SubnetState::Off {
                self.state.set(false);
            }
        }
        
    
        let vals = match self.state.get() {
            true => (SubnetState::On, SubnetState::Off),
            false => (SubnetState::Off, SubnetState::On),
        };
    
        map!(
            3 => vals.0,
            4 => vals.1
        )
    }
}

#[cast_to]
impl Pokeable for DFlipFlop {
    fn pokeable_areas(&self) -> i32 {
        1
    }
    
    fn poke_start(&mut self, _: i32) {}
    
    fn poke_end(&mut self, area: i32) {
        if area == 0 {
            self.state.set(!self.state.get());
        }
    }
}

impl DFlipFlop {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}

#[derive(Debug)]
pub(crate) struct TFlipFlop {
    #[cfg(not(test))]
    state: Cell<bool>,
    #[cfg(test)]
    pub state: Cell<bool>,
}

impl Component for TFlipFlop {
    fn ports(&self) -> usize {
        5 // 0 is T, 1 is clock, 2 is disable, 3 is Q, 4 is Q inverse
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0..=2 => Some(PortType::Input),
            3 | 4 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, data: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let t = port_or_default!(data, 0);
        let clock = data.get(&1).map(|e| e.rising()).unwrap_or(false);
        let disable = port_or_default!(data, 2);
        
        if clock && disable != SubnetState::On {
            if t.truthy() {
                self.state.set(!self.state.get());
            }
        }
        
        let vals = match self.state.get() {
            true => (SubnetState::On, SubnetState::Off),
            false => (SubnetState::Off, SubnetState::On),
        };
        
        map!(
            3 => vals.0,
            4 => vals.1
        )
    }
}

#[cast_to]
impl Pokeable for TFlipFlop {
    fn pokeable_areas(&self) -> i32 {
        1
    }
    
    fn poke_start(&mut self, _: i32) {}
    
    fn poke_end(&mut self, area: i32) {
        if area == 0 {
            self.state.set(!self.state.get());
        }
    }
}

impl TFlipFlop {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}

#[derive(Debug)]
pub(crate) struct JKFlipFlop {
    #[cfg(not(test))]
    state: Cell<bool>,
    #[cfg(test)]
    pub state: Cell<bool>,
}

impl Component for JKFlipFlop {
    fn ports(&self) -> usize {
        6 // 0 is J, 1 is K, 2 is clock, 3 is disable, 4 is Q, 5 is Q inverse
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0..=3 => Some(PortType::Input),
            4 | 5 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, data: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let j = port_or_default!(data, 0);
        let k = port_or_default!(data, 1);
        let clock = data.get(&2).map(|e| e.rising()).unwrap_or(false);
        let disable = port_or_default!(data, 3);
        
        
        if clock && disable != SubnetState::On {
            if j.truthy() && k.falsy() {
                self.state.set(true);
            } else if j.falsy() && k.truthy() {
                self.state.set(false);
            } else if j.truthy() && k.truthy() {
                self.state.set(!self.state.get());
            }
        }
        
        
        let vals = match self.state.get() {
            true => (SubnetState::On, SubnetState::Off),
            false => (SubnetState::Off, SubnetState::On),
        };
        
        map!(
            4 => vals.0,
            5 => vals.1
        )
    }
}

#[cast_to]
impl Pokeable for JKFlipFlop {
    fn pokeable_areas(&self) -> i32 {
        1
    }
    
    fn poke_start(&mut self, _: i32) {}
    
    fn poke_end(&mut self, area: i32) {
        if area == 0 {
            self.state.set(!self.state.get());
        }
    }
}

impl JKFlipFlop {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}

#[derive(Debug)]
pub(crate) struct SRFlipFlop {
    #[cfg(not(test))]
    state: Cell<bool>,
    #[cfg(test)]
    pub state: Cell<bool>,
}

impl Component for SRFlipFlop {
    fn ports(&self) -> usize {
        6 // 0 is S, 1 is R, 2 is clock, 3 is disable, 4 is Q, 5 is Q inverse
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0..=3 => Some(PortType::Input),
            4 | 5 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, data: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let s = port_or_default!(data, 0);
        let r = port_or_default!(data, 1);
        let clock = data.get(&2).map(|e| e.rising()).unwrap_or(false);
        let disable = port_or_default!(data, 3);
        
        
        if clock && disable != SubnetState::On {
            if s.truthy() && r.falsy() {
                self.state.set(true);
            } else if s.falsy() && r.truthy() {
                self.state.set(false);
            }
        }
        
        
        let vals = match self.state.get() {
            true => (SubnetState::On, SubnetState::Off),
            false => (SubnetState::Off, SubnetState::On),
        };
        
        map!(
            4 => vals.0,
            5 => vals.1
        )
    }
}

#[cast_to]
impl Pokeable for SRFlipFlop {
    fn pokeable_areas(&self) -> i32 {
        1
    }
    
    fn poke_start(&mut self, _: i32) {}
    
    fn poke_end(&mut self, area: i32) {
        if area == 0 {
            self.state.set(!self.state.get());
        }
    }
}

impl SRFlipFlop {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}

#[derive(Debug)]
pub(crate) struct Clock {
    state: Cell<bool>,
}

impl Component for Clock {
    fn ports(&self) -> usize {
        1
    }
    
    fn port_type(&self, port: usize) -> Option<PortType> {
        match port {
            0 => Some(PortType::Output),
            _ => None,
        }
    }
    
    fn evaluate(&self, _: HashMap<usize, StateChange>) -> HashMap<usize, SubnetState> {
        let val = match self.state.get() {
            true => SubnetState::On,
            false => SubnetState::Off
        };
        
        self.state.set(!self.state.get());
        
        map!(0 => val)
    }
}

#[cast_to]
impl Pokeable for Clock {
    fn pokeable_areas(&self) -> i32 {
        1
    }
    
    fn poke_start(&mut self, _: i32) {}
    
    fn poke_end(&mut self, area: i32) {
        if area == 0 {
            self.state.set(!self.state.get());
        }
    }
}

impl Clock {
    pub(crate) fn new() -> Self {
        Self { state: Cell::new(false) }
    }
}