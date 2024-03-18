
use ark_bn254::{Fq, G1Projective,Fq2,G2Projective};
use ark_ff::{fields::Field, BigInteger256,One};



fn new_fp_element(x : BigInteger256) -> Fq{
    Fq::from(x)
}

fn new_fp2_element(a: BigInteger256, b: BigInteger256) -> Fq2 {
    Fq2::new(Fq::from(a), Fq::from(b))
}

pub struct G1Point{
    point: G1Projective
}

pub struct G2Point {
    point: G2Projective,
}

impl G2Point {
    // Function to create a new G2Point from x and y coordinates, where each coordinate is a pair of BigIntegers
    pub fn new(x: (BigInteger256, BigInteger256), y: (BigInteger256, BigInteger256)) -> Self {
        // Convert x and y to Fq2 elements
        let x_elem = new_fp2_element(x.1, x.0); 
        let y_elem = new_fp2_element(y.1, y.0);

        // Create a new G2 point in projective coordinates
        let point = G2Projective::new(x_elem, y_elem, Fq2::one()); // Z coordinate is set to 1

        G2Point { point }
    }
}

impl G1Point {
    // Function to create a new G1Point from x and y coordinates
    pub fn new(x: BigInteger256, y: BigInteger256) -> Self {
        // Convert x and y to field elements
        let x_elem = new_fp_element(x);
        let y_elem = new_fp_element(y);

        // Create a new G1 point in projective coordinates
        let point = G1Projective::new(x_elem, y_elem, Fq::one()); // Z coordinate is set to 1

        G1Point { point }
    }
}