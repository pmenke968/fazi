use std::{
    ffi::CStr,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};

use rand::{prelude::SliceRandom, prelude::IteratorRandom, Rng};

use crate::{
    driver::{CONSTANTS, COVERAGE, COVERAGE_BEFORE_ITERATION, FAZI, FAZI_INITIALIZED, LAST_INPUT},
    libfuzzer_runone_fn, signal, Fazi,
};

#[repr(C)]
pub struct FaziInput {
    data: *const u8,
    size: usize,
}

#[no_mangle]
pub extern "C" fn fazi_initialize() {
    if FAZI_INITIALIZED.load(Ordering::Relaxed) {
        return;
    }

    CONSTANTS
        .set(Default::default())
        .expect("CONSTANTS already initialized");
    COVERAGE
        .set(Default::default())
        .expect("COVERAGE already initialized");
    LAST_INPUT
        .set(Default::default())
        .expect("LAST_INPUT already initialized");

    let mut fazi = Fazi::default();

    fazi.restore_inputs();
    fazi.setup_signal_handler();

    FAZI.set(Mutex::new(fazi))
        .expect("FAZI already initialized");

    FAZI_INITIALIZED.store(true, Ordering::Relaxed);
}

#[no_mangle]
pub extern "C" fn fazi_start_testcase() -> FaziInput {
    let fazi = FAZI
        .get()
        .expect("FAZI not initialized")
        .lock()
        .expect("could not lock FAZI");

    FaziInput {
        data: fazi.input.as_ptr(),
        size: fazi.input.len(),
    }
    // .next_iteration(|input| {
    //     let f = libfuzzer_runone_fn();
    //     unsafe {
    //         f(input.as_ptr(), input.len());
    //     }
    // })
}

#[no_mangle]
pub extern "C" fn fazi_end_testcase(need_more_data: bool) {
    let mut fazi = FAZI
        .get()
        .expect("FAZI not initialized")
        .lock()
        .expect("could not lock FAZI");

    fazi.end_iteration(need_more_data);
}

#[no_mangle]
pub extern "C" fn fazi_set_corpus_dir(dir: *const libc::c_char) {
    let mut fazi = FAZI
        .get()
        .expect("FAZI not initialized")
        .lock()
        .expect("could not lock FAZI");

    let dir = unsafe { CStr::from_ptr(dir) };

    fazi.options.corpus_dir = dir.to_string_lossy().into_owned().into();
}

#[no_mangle]
pub extern "C" fn fazi_set_crashes_dir(dir: *const libc::c_char) {
    let mut fazi = FAZI
        .get()
        .expect("FAZI not initialized")
        .lock()
        .expect("could not lock FAZI");

    let dir = unsafe { CStr::from_ptr(dir) };

    fazi.options.crashes_dir = dir.to_string_lossy().into_owned().into();
}

impl<R: Rng> Fazi<R> {
    pub(crate) fn end_iteration(&mut self, need_more_data: bool) {
        let coverage = COVERAGE
            .get()
            .expect("failed to get COVERAGE")
            .lock()
            .expect("failed to lock COVERAGE");
        let new_coverage = coverage.len();
        drop(coverage);

        if !need_more_data {
            let min_input_size = if let Some(min_input_size) = self.min_input_size {
                std::cmp::min(self.input.len(), min_input_size)
            } else {
                self.input.len()
            };

            self.min_input_size = Some(min_input_size);
        }

        let can_request_more_data = self.min_input_size.is_some();

        let old_coverage = COVERAGE_BEFORE_ITERATION.load(Ordering::Relaxed);
        if old_coverage != new_coverage {
            eprintln!(
                "old coverage: {}, new coverage: {}",
                old_coverage, new_coverage
            );

            self.corpus.push(self.input.clone());

            let input = self.input.clone();
            let corpus_dir = self.options.corpus_dir.clone();
            std::thread::spawn(move || {
                signal::save_input(corpus_dir.as_ref(), input.as_slice());
            });
            COVERAGE_BEFORE_ITERATION.store(new_coverage, Ordering::Relaxed);
        } else if !need_more_data || !can_request_more_data {
            if let Some(input) = self
                .corpus
                .iter()
                .filter(|corpus| corpus.len() >= self.min_input_size.unwrap_or(0))
                .choose(&mut self.rng)
            {
                self.input = input.clone();
            }
        }

        if need_more_data && can_request_more_data {
            self.extend_input();
        } else {
            self.mutate_input();
        }

        let mut last_input = LAST_INPUT
            .get()
            .expect("LAST_INPUT not initialized")
            .lock()
            .expect("failed to lock LAST_INPUT");
        *last_input = Arc::clone(&self.input);
        drop(last_input);

        self.iterations += 1;

        if self.iterations % 1000 == 0 {
            eprintln!("iter: {}", self.iterations);
        }
    }
}
