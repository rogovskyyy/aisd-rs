#[allow(unused_imports)]
use crate::algorithms::shellsort::{ ShellSort };
use crate::algorithms::heapsort::{ HeapSort };
//use crate::algorithms::quicksort::{ QuickSort };
use crate::algorithms::quicksortv2::{ QuickSortV2 };
use crate::algorithms::mergesort::{ MergeSort };
use crate::enums::{ Algorithm, Type };
use crate::module::timer::{ Timer };
use crate::module::plotting_engine::{ Plotter };
use rand::Rng;
use std::env;

extern crate time;

pub mod algorithms {
    pub mod heapsort;
    pub mod mergesort;
    pub mod quicksort;
    pub mod quicksortv2;
    pub mod shellsort;
}

mod module {
    pub mod timer;
    pub mod plotting_engine;
}
    
mod enums {
    #[allow(dead_code)]
    pub enum Algorithm {
        Heap, Merge, Quick, Shell
    }
    #[allow(dead_code)]
    pub enum Type {
        Asc, Desc, Random
    }
}

fn call_set(dt: Type, n: u32) -> Vec<u32> {
    match dt {
        Type::Asc => { 
            let counter: u32 = 2u32.pow(n);
            let mut vector: Vec<u32> = Vec::new();
            
            for iter in 0..counter {
                vector.push(iter);
            }

            return vector;
        },
        Type::Desc => { 
            let counter: u32 = 2u32.pow(n);
            let mut vector: Vec<u32> = Vec::new();
            
            for iter in (0..counter).rev() {
                vector.push(iter);
            }

            return vector;
        },
        Type::Random => {
            let counter: u32 = 2u32.pow(n);
            let mut vector: Vec<u32> = Vec::new();

            for _ in 0..counter {
                let mut rng = rand::thread_rng();
                vector.push(rng.gen_range(0..100000));
            }

            return vector;
        }
    };
}

fn call_method(f: Algorithm, dt: Type, n: u32) {

    let data: Vec<u32> = call_set(dt, n);
    let mut bg_data = data.clone();

    let func = |mut arr: &mut Vec<u32>| -> () {
        match f {
            Algorithm::Heap => HeapSort::sort(&mut arr),
            Algorithm::Merge => MergeSort::sort(&mut arr),
            Algorithm::Quick => QuickSortV2::sort(&mut arr),
            Algorithm::Shell => ShellSort::sort(&mut arr)
        }
    };
 
    let start = Timer::get_current_time();
    func(&mut bg_data);
    let stop = Timer::get_current_time();
    println!("{};{};{}", n, 2u32.pow(n), Timer::get_timespan(start, stop));
}

#[allow(unused_doc_comments)]
fn main() {
    
    //println!("2^n;elements;time(ms)");
    //for iter in 1..20 {    
        //call_method(Algorithm::Heap, Type::Asc, iter);
        //call_method(Algorithm::Merge, Type::Asc, iter);
        //call_method(Algorithm::Quick, Type::Asc, iter);
        //call_method(Algorithm::Shell, Type::Asc, iter);

        //call_method(Algorithm::Heap, Type::Desc, iter);
        //call_method(Algorithm::Merge, Type::Desc, iter);
        //call_method(Algorithm::Quick, Type::Desc, iter);
        //call_method(Algorithm::Shell, Type::Desc, iter);

        //call_method(Algorithm::Heap, Type::Random, iter);
        //call_method(Algorithm::Merge, Type::Random, iter);
        //call_method(Algorithm::Quick, Type::Random, iter);
        //call_method(Algorithm::Shell, Type::Random, iter);
    //}
}