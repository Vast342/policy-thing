use crate::{
    arch::{
        lr, BATCHES_PER_SUPERBATCH, BATCH_SIZE, CHECKPOINT_FREQ, NUM_SUPERBATCHES,
        POS_PER_SUPERBATCH, TOTAL_POSITIONS,
    },
    dataloader::Loader,
    inference::{get_gradient, get_loss},
    net::PolicyNetwork,
    types::datapoint::Datapoint,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    mem::size_of,
    time::Instant,
};

pub fn train() {
    // init network
    let mut net = PolicyNetwork::rand();
    // data loader
    let mut loader = Loader::new();
    println!(
        "Total Positions of Data: {}",
        loader.file_size / size_of::<Datapoint>() as u64
    );
    println!(
        "Total Epochs: {}",
        TOTAL_POSITIONS as f64 / (loader.file_size as f64 / size_of::<Datapoint>() as f64)
    );
    // get single batch to calculate loss with
    let mut loss_loader = Loader::new();
    loss_loader.get_position();
    let test_batch = loss_loader.batch;
    let start = Instant::now();
    // train
    for superbatch_num in 0..NUM_SUPERBATCHES {
        let lr = lr(superbatch_num);
        for _batch_num in 0..BATCHES_PER_SUPERBATCH {
            //let batch_start = Instant::now();
            let mut gradient_sum = PolicyNetwork::empty();
            for _position_num in 0..BATCH_SIZE {
                let point = loader.get_position();
                get_gradient(point, &net, &mut gradient_sum);
            }
            gradient_sum /= BATCH_SIZE as f32 / lr;
            net += &gradient_sum;
            //println!("Batch {} done | {} pos/sec", batch_num + 1, BATCH_SIZE as f32 / batch_start.elapsed().as_secs_f32());
        }
        println!(
            "Superbatch {} done | {} seconds | {} pos/sec | loss {}",
            superbatch_num + 1,
            start.elapsed().as_secs_f32(),
            (POS_PER_SUPERBATCH * (superbatch_num + 1)) as f32 / start.elapsed().as_secs_f32(),
            get_run_loss(&test_batch, &net)
        );
        if (superbatch_num + 1) % CHECKPOINT_FREQ == 0 {
            let mut writer = BufWriter::new(
                File::create(format!("apn_003-{}.pn", superbatch_num + 1))
                    .expect("couldn't create file"),
            );
            unsafe {
                writer
                    .write_all(any_as_u8_slice(net.as_ref()))
                    .expect("failed to write to file");
            }
        }
    }
    // save to a file
    let mut writer = BufWriter::new(File::create("apn_003.pn").expect("couldn't create file"));
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

pub fn get_run_loss(batch: &Box<[Datapoint]>, net: &Box<PolicyNetwork>) -> f32 {
    let mut loss: f32 = 0.0;
    for position_num in 0..BATCH_SIZE {
        loss += get_loss(batch[position_num], &net).powf(2.0);
    }
    (1.0 / BATCH_SIZE as f32) * loss
}
