use std::time::Instant;

use rand::Rng;
use rand::prelude::ThreadRng;

use do_util::pareto_pq::{
    ParetoElement, ParetoFront,
    util::{CartesianParetoElement, export_2d_visualization}
};
use do_util::pareto_pq::list::ListParetoFront;
// use do_util::pareto_pq::naive_kd_tree::NaiveKDTreeFront;
use do_util::pareto_pq::kd_tree::KDTreeFront;

pub fn random_element<const NB_DIM:usize>(rng:&mut ThreadRng) -> CartesianParetoElement<NB_DIM> {
    let mut coords = [0;NB_DIM];
    loop {
        for i in coords.iter_mut().take(NB_DIM) {
            *i = rng.gen();
        }
        if coords.iter().map(|e| {
            (*e as f64 / u16::MAX as f64).powf(NB_DIM as f64)
        }).sum::<f64>() <= 1. {
            let mut new_coords = [u16::MAX ; NB_DIM];
            for i in 0..NB_DIM {
                new_coords[i] -= coords[i];
            }
            return CartesianParetoElement::new(new_coords);
        }
    }
    
}


pub fn perform_bench<'a, T, Elt:ParetoElement<T, NB_DIM>, It, Front, const NB_DIM:usize>
(elements:&[Elt], mut front:Front) where
T:Ord,
Elt:'a + ParetoElement<T,NB_DIM>+Eq+Clone,
It:Iterator<Item=&'a Elt>,
Front:ParetoFront<'a, T, Elt, It, NB_DIM> {
    // inserts elements
    let start_insert = Instant::now();
    for e in elements {
        front.insert(e.clone());
    }
    let time_insert = start_insert.elapsed().as_secs_f32();
    println!("\t{:<5} inserts in {} seconds ({} inserts/s)",
        elements.len(), time_insert, elements.len() as f32/time_insert
    );
    // pop them until no more elements in the front
    let mut nb_pops:usize = 0;
    let start_pop = Instant::now();
    while front.pop_minimum_element(0).is_some() {
        nb_pops += 1;
    }
    let time_pop = start_pop.elapsed().as_secs_f32();
    println!("\t{:<5} pops in {} seconds ({} pops/s)", nb_pops, time_pop, elements.len() as f32/time_pop);
}


pub fn bench_pareto<const NB_DIM:usize>(nb_elts:usize) {
    let mut rng = rand::thread_rng();
    let mut elements:Vec<CartesianParetoElement<NB_DIM>> = Vec::new();
    for _ in 0..nb_elts {
        elements.push(random_element(&mut rng));
    }
    // TEST LIST PARETO
    println!("List structure:");
    let list_front:ListParetoFront<CartesianParetoElement<NB_DIM>> = ListParetoFront::default();
    perform_bench(&elements, list_front);
    // // TEST NAIVE KD-TREE
    // println!("Naive kd-tree structure:");
    // let naive_kdtree_front:NaiveKDTreeFront<u16, CartesianParetoElement<NB_DIM>, NB_DIM> = NaiveKDTreeFront::default();
    // perform_bench(&elements, naive_kdtree_front);
    // TEST KD-TREE
    println!("kd-tree structure:");
    let kdtree_front:KDTreeFront<u16, CartesianParetoElement<NB_DIM>, NB_DIM> = KDTreeFront::default();
    perform_bench(&elements, kdtree_front);
}

pub fn bench_list_2d(nb_elts:usize) {
    let mut rng = rand::thread_rng();
    let mut elements:Vec<CartesianParetoElement<2>> = Vec::new();
    for _ in 0..nb_elts {
        elements.push(random_element(&mut rng));
    }

    let mut front:ListParetoFront<CartesianParetoElement<2>> = ListParetoFront::default();
    // inserts elements
    let start_insert = Instant::now();
    for e in elements {
        front.insert(e.clone());
    }
    let time_insert = start_insert.elapsed().as_secs_f32();
    println!("{:<5} inserts in {} seconds ({} inserts/s)", nb_elts, time_insert, nb_elts as f32/time_insert);
    // export 2d viz
    export_2d_visualization(&front, "tmp/2dviz.vg.json");
    // pop them until no more elements in the front
    let mut nb_pops:usize = 0;
    let start_pop = Instant::now();
    while front.pop_minimum_element(0).is_some() {
        nb_pops += 1;
    }
    let time_pop = start_pop.elapsed().as_secs_f32();
    println!("{:<5} pops in {} seconds ({} pops/s)", nb_pops, time_pop, nb_elts as f32/time_pop);
}

pub fn main() {
    // bench_list_2d(1000000);
    bench_pareto::<2>(10000000);
}