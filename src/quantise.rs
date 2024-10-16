use core::f32;
use std::{fs::File, io::{BufWriter, Write}};

use crate::{arch::OUTPUT_SIZE, net::{PolicyNetwork, QuantisedPolicyNetwork, OW_SIZE}};

// uncomment if you are using it
static NET_FILE: PolicyNetwork = unsafe { std::mem::transmute(*include_bytes!("apn_003.pn")) };

pub const LIMIT: i16 = i16::MAX;

pub fn quantise_from_file() {
    // find optimal QA value
    let mut best_qa = f32::INFINITY;
    for i in 0..OW_SIZE {
        let qa = LIMIT as f32 / NET_FILE.output_weights[i].abs();
        if qa < best_qa {
            best_qa = qa;
        }
    }
    for i in 0..OUTPUT_SIZE {
        let qa = LIMIT as f32 / NET_FILE.output_biases[i].abs();
        if qa < best_qa {
            best_qa = qa;
        }
    }
    best_qa = best_qa.ceil();
    println!("found optimal QA, it's {best_qa}");
    let net = quantise(Box::new(NET_FILE), best_qa);
    // save to a file
    let mut writer = BufWriter::new(File::create("apn_003-q.pn").expect("couldn't create file"));
    unsafe {
        writer
            .write_all(any_as_u8_slice(net.as_ref()))
            .expect("failed to write to file");
    }
}

// thank you stack overflow :)
unsafe fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
}

pub fn quantise(net: Box<PolicyNetwork>, qa: f32) -> Box<QuantisedPolicyNetwork> {
    let mut q_net = QuantisedPolicyNetwork::empty();
    for i in 0..OW_SIZE {
        q_net.output_weights[i] = (net.output_weights[i] * qa).round() as i16;
    }
    for i in 0..OUTPUT_SIZE {
        q_net.output_biases[i] = (net.output_biases[i] * qa).round() as i16;
    }
    q_net
}
