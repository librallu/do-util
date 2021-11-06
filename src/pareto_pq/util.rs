use std::{fs::File, io::Write};

use serde_json::json;

use crate::pareto_pq::{ParetoElement, ParetoFront};


/// Simple 2D point for the pareto front
#[derive(Debug,Clone,Eq,PartialEq)]
pub struct CartesianParetoElement<const NB_DIM:usize> {
    /// coordinates
    coords:[u16;NB_DIM]
}

impl<const NB_DIM:usize> ParetoElement<u16,NB_DIM> for CartesianParetoElement<NB_DIM> {
    fn coordinates(&self) -> &[u16;NB_DIM] { &self.coords }

    fn dominates(&self, other:&Self) -> bool {
        for i in 0..NB_DIM {
            if self.coords[i] > other.coords[i] { return false; }
        }
        true
    }
}

impl<const NB_DIM:usize> CartesianParetoElement<NB_DIM> {
    /// constructor taking two coordinates
    pub fn new(coords:[u16;NB_DIM]) -> Self {
        Self { coords }
    }
}


/// generates 2D visualization of a pareto front using vega-lite
pub fn generate_2d_visualization<
    'a,
    T:Ord+Into<f64>+Copy,
    Elt:'a+ParetoElement<T,2>+Eq,
    It:Iterator<Item=&'a Elt>,
    Front:ParetoFront<'a, T, Elt, It, 2>
>(front:&'a Front) -> String {
    // create data values
    let mut values:Vec<serde_json::Value> = Vec::new();
    for elt in front.iter() {
        values.push(json!({"x":elt.coordinates()[0].into(), "y":elt.coordinates()[1].into()}));
    }
    // create vegalite json object
    let json_res = json!({
        "$schema": "https://vega.github.io/schema/vega-lite/v5.json",
        "title": "node feature visualization",
        "width": 500,
        "height": 250,
        "data": {
            "values": values
        },
        "encoding": {
            "x": {
                "field": "x",
                "title": "x",
                "type": "quantitative"
            },
            "y": {
                "field": "y",
                "title": "y",
                "type": "quantitative"
            }
        },
        "layer": [
            {"mark": {"type": "point", "shape": "circle", "filled": true}}
        ]
    });
    serde_json::to_string_pretty(&json_res).unwrap()
}

/// exports vega-lite 2d pareto front in a file
pub fn export_2d_visualization<
    'a,
    T:Ord+Into<f64>+Copy,
    Elt:'a+ParetoElement<T,2>+Eq,
    It:Iterator<Item=&'a Elt>,
    Front:ParetoFront<'a, T, Elt, It, 2>
>(front:&'a Front, filename:&str) {
    let mut file = File::create(filename)
        .expect("unable to open the file");
    file.write_all(generate_2d_visualization(front).as_bytes())
        .expect("unable to write file content");
} 



#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn test_strict_dominance() {
        let e1 = CartesianParetoElement::new([0]);
        let e2 = CartesianParetoElement::new([1]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_nonstrict_dominance() {
        let e1 = CartesianParetoElement::new([0]);
        let e2 = CartesianParetoElement::new([0]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_non_dominance() {
        let e1 = CartesianParetoElement::new([1]);
        let e2 = CartesianParetoElement::new([0]);
        assert!(!e1.dominates(&e2));
    }

    #[test]
    pub fn test_2d_dominance() {
        let e1 = CartesianParetoElement::new([1,0]);
        let e2 = CartesianParetoElement::new([2,1]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_2d_dominance2() {
        let e1 = CartesianParetoElement::new([1,0]);
        let e2 = CartesianParetoElement::new([1,1]);
        assert!(e1.dominates(&e2));
    }

    #[test]
    pub fn test_2d_non_dominance() {
        let e1 = CartesianParetoElement::new([1,0]);
        let e2 = CartesianParetoElement::new([0,1]);
        assert!(!e1.dominates(&e2));
    }
}