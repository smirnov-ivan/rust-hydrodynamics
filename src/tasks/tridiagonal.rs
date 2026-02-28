use crate::core::linalg::tridiagonal_system::TridiagonalSystem;
use crate::core::linalg::vector::Vector;
use crate::core::math::lfloat::Lfloat;
use serde::{Serialize, Deserialize};
use std::fmt::Display;

#[derive(Deserialize)]
pub struct TestTridiagonalParams {
    id: String,
}

#[derive(Serialize)]
pub struct TestTridiagonalResult {
    th1: bool,
    th2: bool,
    result: Vector<Lfloat>,
    residual: Vector<Lfloat>,
    norm: Lfloat
}


pub struct TridiagonalTasks {}

impl TridiagonalTasks {

    pub fn test(params: &TestTridiagonalParams) -> Result<TestTridiagonalResult, &'static str> {
        let path = format!("tests/tridiagonal/{}.txt", params.id);
        let system: TridiagonalSystem<Lfloat> = TridiagonalSystem::load(&path).expect("Error loading test");
        let t1 = system.checkT1();
        let t2 = system.checkT2();
        let result = system.solve().expect("Can't solve system");
        let operated = &system * &result;
        let rightSide = system.getRight();
        let residual = &operated - &rightSide;
        let norm = residual.norm();
        Ok(TestTridiagonalResult {
            th1: t1,
            th2: t2,
            result: result,
            residual: residual,
            norm: norm
        })
    }

}