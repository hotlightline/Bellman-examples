#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


use rand_core::OsRng;
use ff::*;
use pairing::Engine;
use pairing::bls12_381::Bls12;
use bellman::{Circuit, ConstraintSystem, SynthesisError};
use bellman::groth16::{
    create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof, Proof,
};

// proving that I know x such that x^3 + x + 5 == 35
// Generalized: x^3 + x + 5 == out
pub struct CubeDemo<E: Engine> {
    x: Option<E::Fr>,
}

impl <E: Engine> Circuit<E> for CubeDemo<E> {
    fn synthesize<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        self.simple2(cs)?;
        Ok(())
    }
}


impl <E: Engine>  CubeDemo<E> {
    //简单版本的写法
    fn simple<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        println!("simple 1");
        let x_val = self.x;
        let tmp_19 = x_val.map(|mut e| {
            e.square();
            e.mul_assign(&x_val.unwrap());
            e.add_assign(&x_val.unwrap());
            e.add_assign(&<E as ff::ScalarEngine>::Fr::from_str("5").unwrap());
            e
        });

        //右边input
        let sox= cs.alloc_input(|| "out", ||{
            let  tmp = tmp_19.unwrap();
            Ok(tmp_19.unwrap())
        })?;
        
        let input1 = cs.alloc(|| "a", || {
            tmp_19.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        cs.enforce(
            || "XXX",
            |lc| lc + input1,
            |lc| lc + CS::one(),
            |lc| lc + sox
        );
    
        Ok(())
    }
    
    fn simple2<CS: ConstraintSystem<E>>(
        self,
        cs: &mut CS
    ) -> Result<(), SynthesisError>
    {
        println!("simple 2");
        let x_val = self.x;
        let tmp_19 = x_val.map(|mut e| {
            e.square();
            e.mul_assign(&x_val.unwrap());
            e.add_assign(&x_val.unwrap());
            e.add_assign(&<E as ff::ScalarEngine>::Fr::from_str("5").unwrap());
            e
        });
        
        //右边input
        let sox= cs.alloc(|| "out", ||{
            let  tmp = tmp_19.unwrap();
            Ok(tmp_19.unwrap())
        })?;
        
        let input1 = cs.alloc_input(|| "a", || {
            tmp_19.ok_or(SynthesisError::AssignmentMissing)
        })?;
        
        cs.enforce(
            || "XXX",
            |lc| lc + input1,
            |lc| lc + CS::one(),
            |lc| lc + sox
        );
    
        Ok(())
    }
}

#[test]
fn test_cube_proof(){
    // This may not be cryptographically safe, use
    // `OsRng` (for example) in production software.
    let rng = &mut OsRng;
    println!("Creating parameters...");
    // Create parameters for our circuit
    let params = {
        let c = CubeDemo::<Bls12> {
            x: None
        };

        generate_random_parameters(c, rng).unwrap()
    };

    // Prepare the verification key (for proof verification)
    let pvk = prepare_verifying_key(&params.vk);

    println!("Creating proofs...");

    // Create an instance of circuit
    let c = CubeDemo::<Bls12> {
        x: <Bls12 as ScalarEngine>::Fr::from_str("3")
    };
    // Create a groth16 proof with our parameters.
    let proof = create_random_proof(c, &params, rng).unwrap();

    assert!(verify_proof(
        &pvk,
        &proof,
        &[<Bls12 as ScalarEngine>::Fr::from_str("35").unwrap()]
    ).unwrap());
}