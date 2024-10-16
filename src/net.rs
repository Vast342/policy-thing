use crate::arch::{INPUT_SIZE, OUTPUT_SIZE};
use bytemuck::{Pod, Zeroable};
use rand::Rng;
use std::ops::{AddAssign, DivAssign, Mul};
pub const OW_SIZE: usize = INPUT_SIZE * OUTPUT_SIZE;
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct PolicyNetwork {
    pub output_weights: [f32; OW_SIZE],
    pub output_biases: [f32; OUTPUT_SIZE],
}

unsafe impl Pod for PolicyNetwork {}
unsafe impl Zeroable for PolicyNetwork {}

impl AddAssign<&PolicyNetwork> for Box<PolicyNetwork> {
    fn add_assign(&mut self, rhs: &PolicyNetwork) {
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            self.output_weights[i] += rhs.output_weights[i];
        }
        for i in 0..OUTPUT_SIZE {
            self.output_biases[i] += rhs.output_biases[i];
        }
    }
}

impl DivAssign<f32> for Box<PolicyNetwork> {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            self.output_weights[i] /= rhs;
        }
        for i in 0..OUTPUT_SIZE {
            self.output_biases[i] /= rhs;
        }
    }
}
impl Mul<f32> for Box<PolicyNetwork> {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        let mut net = PolicyNetwork::empty();
        for i in 0..(INPUT_SIZE * OUTPUT_SIZE) {
            net.output_weights[i] = self.output_weights[i] * rhs;
        }
        for i in 0..OUTPUT_SIZE {
            net.output_biases[i] = self.output_biases[i] * rhs;
        }
        net
    }
}
impl PolicyNetwork {
    // Constructs a randomized heap-allocated network
    pub fn rand() -> Box<Self> {
        let mut rng = rand::thread_rng();
        let mut network = Box::new(PolicyNetwork {
            output_weights: [0.0; OW_SIZE],
            output_biases: [0.0; OUTPUT_SIZE],
        });

        for weight in network.output_weights.iter_mut() {
            *weight = rng.gen_range(-1.0..1.0);
        }

        for bias in network.output_biases.iter_mut() {
            *bias = rng.gen_range(-1.0..1.0);
        }

        network
    }

    // Constructs a zeroed heap-allocated network
    pub fn empty() -> Box<Self> {
        // Use bytemuck to create a zeroed instance
        Box::new(Self::zeroed())
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct QuantisedPolicyNetwork {
    pub output_weights: [i16; OW_SIZE],
    pub output_biases: [i16; OUTPUT_SIZE],
}

unsafe impl Pod for QuantisedPolicyNetwork {}
unsafe impl Zeroable for QuantisedPolicyNetwork {}

impl QuantisedPolicyNetwork {
    // Constructs a zeroed heap-allocated network
    pub fn empty() -> Box<Self> {
        // Use bytemuck to create a zeroed instance
        Box::new(Self::zeroed())
    }
}