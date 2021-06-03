/************************************************************************
************************************************************************
    FAUST Architecture File
    Copyright (C) 2017-2020 GRAME, Centre National de Creation Musicale
    ---------------------------------------------------------------------

    This is sample code. This file is provided as an example of minimal
    FAUST architecture file. Redistribution and use in source and binary
    forms, with or without modification, in part or in full are permitted.
    In particular you can create a derived work of this FAUST architecture
    and distribute that work under terms of your choice.

    This sample code is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

//! Faust JACK architecture file
extern crate jack;
use jack::prelude as j;
use std::io;
extern crate libm;

type F32 = f32;
type F64 = f64;

#[derive(Copy, Clone)]
pub struct ParamIndex(pub i32);

pub struct Soundfile<'a> {
    fBuffers: &'a&'a F32,
    fLength: &'a i32,
    fSR: &'a i32,
    fOffset: &'a i32,
    fChannels: i32
}

pub trait FaustDsp {
    type T;

    fn new() -> Self where Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32) where Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_horizontal_slider(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}



pub struct mydspSIG0 {
	iVec0: [i32;2],
	iRec0: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		let mut l0: i32 = 0;
		loop {
			self.iVec0[l0 as usize] = 0;
			l0 = (l0 + 1);
			if (l0 < 2) { continue; } else { break; }
		}
		let mut l1: i32 = 0;
		loop {
			self.iRec0[l1 as usize] = 0;
			l1 = (l1 + 1);
			if (l1 < 2) { continue; } else { break; }
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i in 0..count {
			self.iVec0[0] = 1;
			self.iRec0[0] = ((self.iVec0[1] + self.iRec0[1]) % 65536);
			table[i as usize] = F32::sin((9.58738019e-05 * (self.iRec0[0] as F32)));
			self.iVec0[1] = self.iVec0[0];
			self.iRec0[1] = self.iRec0[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iVec0: [0;2],
		iRec0: [0;2],
	}
}
static mut ftbl0mydspSIG0: [F32;65536] = [0.0;65536];
pub struct mydsp {
	fSampleRate: i32,
	fConst0: F32,
	fRec1: [F32;2],
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			fSampleRate: 0,
			fConst0: 0.0,
			fRec1: [0.0;2],
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", "Faust Basic Element Library");
		m.declare("basics.lib/version", "0.1");
		m.declare("filename", "exfaust1.dsp");
		m.declare("maths.lib/author", "GRAME");
		m.declare("maths.lib/copyright", "GRAME");
		m.declare("maths.lib/license", "LGPL with exception");
		m.declare("maths.lib/name", "Faust Math Library");
		m.declare("maths.lib/version", "2.3");
		m.declare("name", "exfaust1");
		m.declare("oscillators.lib/name", "Faust Oscillator Library");
		m.declare("oscillators.lib/version", "0.1");
		m.declare("platform.lib/name", "Generic Platform Library");
		m.declare("platform.lib/version", "0.1");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 1;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(65536, unsafe { &mut ftbl0mydspSIG0 });
	}
	fn instance_reset_params(&mut self) {
	}
	fn instance_clear(&mut self) {
		for l2 in 0..2 {
			self.fRec1[l2 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = (440.0 / F32::min(192000.0, F32::max(1.0, (self.fSampleRate as F32))));
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("exfaust1");
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0) = if let [outputs0, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			(outputs0)
		} else {
			panic!("wrong number of outputs");
		};
		let zipped_iterators = outputs0;
		for output0 in zipped_iterators {
			self.fRec1[0] = (self.fConst0 + (self.fRec1[1] - F32::floor((self.fConst0 + self.fRec1[1]))));
			*output0 = ((0.200000003 * unsafe { ftbl0mydspSIG0[((65536.0 * self.fRec1[0]) as i32) as usize] }) as F32);
			self.fRec1[1] = self.fRec1[0];
		}
	}

}


fn main() {

    // Create JACK client
    let (client, _status) = j::Client::new("faust_rust", j::client_options::NO_START_SERVER).unwrap();

    // Allocation DSP on the heap
    let mut dsp = Box::new(mydsp::new());

    println!("Faust Rust code running with JACK: sample-rate = {} buffer-size = {}", client.sample_rate(), client.buffer_size());

    println!("get_num_inputs: {}", dsp.get_num_inputs());
    println!("get_num_outputs: {}", dsp.get_num_outputs());

    // Init DSP with a given SR
    dsp.init(client.sample_rate() as i32);

    // Register ports. They will be used in a callback that will be
    // called when new data is available.

    let in_a = client.register_port("in1", j::AudioInSpec::default()).unwrap();
    let in_b = client.register_port("in2", j::AudioInSpec::default()).unwrap();

    let mut out_a = client.register_port("out1", j::AudioOutSpec::default()).unwrap();
    let mut out_b = client.register_port("out2", j::AudioOutSpec::default()).unwrap();

    let process_callback = move |_: &j::Client, ps: &j::ProcessScope| -> j::JackControl {
        let mut out_a_p = j::AudioOutPort::new(&mut out_a, ps);
        let mut out_b_p = j::AudioOutPort::new(&mut out_b, ps);

        let in_a_p = j::AudioInPort::new(&in_a, ps);
        let in_b_p = j::AudioInPort::new(&in_b, ps);

        let input0: &[f32] = &in_a_p;
        let input1: &[f32] = &in_b_p;

        let output0: &mut[f32] = &mut out_a_p;
        let output1: &mut[f32] = &mut out_b_p;

        let inputs = &[input0, input1];
        let outputs = &mut[output0, output1];

        dsp.compute(in_a_p.len() as i32, inputs, outputs);

        j::JackControl::Continue
    };
    let process = j::ClosureProcessHandler::new(process_callback);

    // Activate the client, which starts the processing.
    let active_client = j::AsyncClient::new(client, (), process).unwrap();

    // Wait for user input to quit
    println!("Press enter/return to quit...");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).ok();

    active_client.deactivate().unwrap();
}
