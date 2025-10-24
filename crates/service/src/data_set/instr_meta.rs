use crate::types_analyzer::{HeapType, OperandType, RefType, ResolvedSig, ValType};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{collections::HashMap, sync::LazyLock};

pub(crate) static INSTR_SIG: LazyLock<FxHashMap<&'static str, ResolvedSig>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity_and_hasher(451, FxBuildHasher);
    map.insert(
        "unreachable",
        ResolvedSig {
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "nop",
        ResolvedSig {
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "drop",
        ResolvedSig {
            params: vec![OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "local.get",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "local.set",
        ResolvedSig {
            params: vec![OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "local.tee",
        ResolvedSig {
            params: vec![OperandType::Any],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "global.get",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "global.set",
        ResolvedSig {
            params: vec![OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "table.get",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "table.set",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32), OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "i32.load",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.load",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.load",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.load",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.load8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load8_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load16_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.load8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load8_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load16_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i32.store",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "f32.store",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "f64.store",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::F64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i32.store8",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i32.store16",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store8",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store16",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store32",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "memory.size",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "memory.grow",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.const",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.const",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.const",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.const",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.eqz",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.lt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.lt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.gt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.gt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.le_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.le_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ge_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ge_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.eqz",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.lt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.lt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.gt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.gt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.le_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.le_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.ge_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.ge_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.lt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.gt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.le",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.ge",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.lt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.gt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.le",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.ge",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.clz",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ctz",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.popcnt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.div_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.div_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rem_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rem_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.and",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.or",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.xor",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.shl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.shr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.shr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rotl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rotr",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.clz",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.ctz",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.popcnt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.div_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.div_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rem_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rem_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.and",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.or",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.xor",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.shl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.shr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.shr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rotl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rotr",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.ceil",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.floor",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.trunc",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.nearest",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.sqrt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.div",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.min",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.max",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.copysign",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.ceil",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.floor",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.trunc",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.nearest",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.sqrt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.div",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.min",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.max",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.copysign",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.wrap_i64",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f64_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f64_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.extend_i32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend_i32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f64_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f64_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.convert_i32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i64_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i64_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.demote_f64",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.convert_i32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i64_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i64_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.promote_f32",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.reinterpret_f32",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.reinterpret_f64",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.reinterpret_i32",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.reinterpret_i64",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.extend8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.extend16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.extend8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "ref.null",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "ref.is_null",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Any,
                nullable: true,
            }))],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "ref.func",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Func,
                nullable: false,
            }))],
        },
    );
    map.insert(
        "ref.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Eq,
                    nullable: true,
                })),
                OperandType::Val(ValType::Ref(RefType {
                    heap_ty: HeapType::Eq,
                    nullable: true,
                })),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "array.len",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Array,
                nullable: true,
            }))],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "any.convert_extern",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Extern,
                nullable: true,
            }))],
            results: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Any,
                nullable: true,
            }))],
        },
    );
    map.insert(
        "extern.convert_any",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Any,
                nullable: true,
            }))],
            results: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::Extern,
                nullable: true,
            }))],
        },
    );
    map.insert(
        "ref.i31",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::I31,
                nullable: false,
            }))],
        },
    );
    map.insert(
        "i31.get_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::I31,
                nullable: true,
            }))],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i31.get_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::Ref(RefType {
                heap_ty: HeapType::I31,
                nullable: true,
            }))],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f64_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f64_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.trunc_sat_f32_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f32_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f64_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f64_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "memory.init",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "data.drop",
        ResolvedSig {
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "memory.copy",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "memory.fill",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "table.init",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "elem.drop",
        ResolvedSig {
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "table.copy",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "table.grow",
        ResolvedSig {
            params: vec![OperandType::Any, OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "table.size",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "table.fill",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Any,
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.load",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8x8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8x8_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32x2_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32x2_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8_splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16_splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32_splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.store",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.const",
        ResolvedSig {
            params: vec![],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shuffle",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.swizzle",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.splat",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.extract_lane_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.extract_lane_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.replace_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extract_lane_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.extract_lane_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.replace_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extract_lane",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.replace_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extract_lane",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64x2.replace_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.extract_lane",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32x4.replace_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.extract_lane",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64x2.replace_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.lt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.lt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.gt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.gt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.le_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.le_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.ge_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.ge_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.lt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.lt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.gt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.gt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.le_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.le_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.ge_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.ge_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.lt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.lt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.gt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.gt_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.le_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.le_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.ge_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.ge_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.lt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.gt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.le",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.ge",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.lt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.gt",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.le",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.ge",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.not",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.and",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.andnot",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.or",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.xor",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.bitselect",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.any_true",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "v128.load8_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.store8_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.store16_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.store32_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.store64_lane",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.load32_zero",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_zero",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.demote_f64x2_zero",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.promote_low_f32x4",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.popcnt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.all_true",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.bitmask",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.narrow_i16x8_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.narrow_i16x8_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.ceil",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.floor",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.trunc",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.nearest",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.add_sat_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.add_sat_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.sub_sat_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.sub_sat_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.ceil",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.floor",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.min_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.min_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.max_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.max_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.trunc",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.avgr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extadd_pairwise_i8x16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extadd_pairwise_i8x16_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extadd_pairwise_i16x8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extadd_pairwise_i16x8_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.q15mulr_sat_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.all_true",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.bitmask",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.narrow_i32x4_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.narrow_i32x4_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_low_i8x16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_high_i8x16_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_low_i8x16_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_high_i8x16_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.add_sat_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.add_sat_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.sub_sat_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.sub_sat_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.nearest",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.min_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.min_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.max_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.max_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.avgr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_low_i8x16_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_high_i8x16_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_low_i8x16_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_high_i8x16_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.all_true",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.bitmask",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.extend_low_i16x8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_high_i16x8_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_low_i16x8_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_high_i16x8_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.min_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.min_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.max_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.max_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.dot_i16x8_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_low_i16x8_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_high_i16x8_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_low_i16x8_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_high_i16x8_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.all_true",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64x2.bitmask",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64x2.extend_low_i32x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_high_i32x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_low_i32x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_high_i32x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shl",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shr_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.eq",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.ne",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.lt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.gt_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.le_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.ge_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_low_i32x4_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_high_i32x4_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_low_i32x4_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_high_i32x4_u",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.sqrt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.div",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.min",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.max",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.pmin",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.pmax",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.abs",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.neg",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.sqrt",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.add",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.sub",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.mul",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.div",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.min",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.max",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.pmin",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.pmax",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f32x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f32x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.convert_i32x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.convert_i32x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f64x2_s_zero",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f64x2_u_zero",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.convert_low_i32x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.convert_low_i32x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.relaxed_swizzle",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.relaxed_trunc_f32x4_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.relaxed_trunc_f32x4_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.relaxed_trunc_f64x2_s",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.relaxed_trunc_f64x2_u",
        ResolvedSig {
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.relaxed_madd",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.relaxed_nmadd",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.relaxed_madd",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.relaxed_nmadd",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.relaxed_laneselect",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.relaxed_laneselect",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.relaxed_laneselect",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.relaxed_laneselect",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.relaxed_min",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.relaxed_max",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.relaxed_min",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.relaxed_max",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.relaxed_q15mulr_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.relaxed_dot_i8x16_i7x16_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.relaxed_dot_i8x16_i7x16_add_s",
        ResolvedSig {
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map
});

pub(crate) static INSTR_OP_CODES: LazyLock<FxHashMap<&'static str, u32>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity_and_hasher(499, FxBuildHasher);
    map.insert("unreachable", 0x00);
    map.insert("nop", 0x01);
    map.insert("block", 0x02);
    map.insert("loop", 0x03);
    map.insert("if", 0x04);
    map.insert("else", 0x05);
    map.insert("throw", 0x08);
    map.insert("throw_ref", 0x0A);
    map.insert("end", 0x0B);
    map.insert("br", 0x0C);
    map.insert("br_if", 0x0D);
    map.insert("br_table", 0x0E);
    map.insert("return", 0x0F);
    map.insert("call", 0x10);
    map.insert("call_indirect", 0x11);
    map.insert("return_call", 0x12);
    map.insert("return_call_indirect", 0x13);
    map.insert("call_ref", 0x14);
    map.insert("return_call_ref", 0x15);
    map.insert("drop", 0x1A);
    map.insert("select", 0x1B);
    map.insert("select.", 0x1C);
    map.insert("try_table", 0x1F);
    map.insert("local.get", 0x20);
    map.insert("local.set", 0x21);
    map.insert("local.tee", 0x22);
    map.insert("global.get", 0x23);
    map.insert("global.set", 0x24);
    map.insert("table.get", 0x25);
    map.insert("table.set", 0x26);
    map.insert("i32.load", 0x28);
    map.insert("i64.load", 0x29);
    map.insert("f32.load", 0x2A);
    map.insert("f64.load", 0x2B);
    map.insert("i32.load8_s", 0x2C);
    map.insert("i32.load8_u", 0x2D);
    map.insert("i32.load16_s", 0x2E);
    map.insert("i32.load16_u", 0x2F);
    map.insert("i64.load8_s", 0x30);
    map.insert("i64.load8_u", 0x31);
    map.insert("i64.load16_s", 0x32);
    map.insert("i64.load16_u", 0x33);
    map.insert("i64.load32_s", 0x34);
    map.insert("i64.load32_u", 0x35);
    map.insert("i32.store", 0x36);
    map.insert("i64.store", 0x37);
    map.insert("f32.store", 0x38);
    map.insert("f64.store", 0x39);
    map.insert("i32.store8", 0x3A);
    map.insert("i32.store16", 0x3B);
    map.insert("i64.store8", 0x3C);
    map.insert("i64.store16", 0x3D);
    map.insert("i64.store32", 0x3E);
    map.insert("memory.size", 0x3F);
    map.insert("memory.grow", 0x40);
    map.insert("i32.const", 0x41);
    map.insert("i64.const", 0x42);
    map.insert("f32.const", 0x43);
    map.insert("f64.const", 0x44);
    map.insert("i32.eqz", 0x45);
    map.insert("i32.eq", 0x46);
    map.insert("i32.ne", 0x47);
    map.insert("i32.lt_s", 0x48);
    map.insert("i32.lt_u", 0x49);
    map.insert("i32.gt_s", 0x4A);
    map.insert("i32.gt_u", 0x4B);
    map.insert("i32.le_s", 0x4C);
    map.insert("i32.le_u", 0x4D);
    map.insert("i32.ge_s", 0x4E);
    map.insert("i32.ge_u", 0x4F);
    map.insert("i64.eqz", 0x50);
    map.insert("i64.eq", 0x51);
    map.insert("i64.ne", 0x52);
    map.insert("i64.lt_s", 0x53);
    map.insert("i64.lt_u", 0x54);
    map.insert("i64.gt_s", 0x55);
    map.insert("i64.gt_u", 0x56);
    map.insert("i64.le_s", 0x57);
    map.insert("i64.le_u", 0x58);
    map.insert("i64.ge_s", 0x59);
    map.insert("i64.ge_u", 0x5A);
    map.insert("f32.eq", 0x5B);
    map.insert("f32.ne", 0x5C);
    map.insert("f32.lt", 0x5D);
    map.insert("f32.gt", 0x5E);
    map.insert("f32.le", 0x5F);
    map.insert("f32.ge", 0x60);
    map.insert("f64.eq", 0x61);
    map.insert("f64.ne", 0x62);
    map.insert("f64.lt", 0x63);
    map.insert("f64.gt", 0x64);
    map.insert("f64.le", 0x65);
    map.insert("f64.ge", 0x66);
    map.insert("i32.clz", 0x67);
    map.insert("i32.ctz", 0x68);
    map.insert("i32.popcnt", 0x69);
    map.insert("i32.add", 0x6A);
    map.insert("i32.sub", 0x6B);
    map.insert("i32.mul", 0x6C);
    map.insert("i32.div_s", 0x6D);
    map.insert("i32.div_u", 0x6E);
    map.insert("i32.rem_s", 0x6F);
    map.insert("i32.rem_u", 0x70);
    map.insert("i32.and", 0x71);
    map.insert("i32.or", 0x72);
    map.insert("i32.xor", 0x73);
    map.insert("i32.shl", 0x74);
    map.insert("i32.shr_s", 0x75);
    map.insert("i32.shr_u", 0x76);
    map.insert("i32.rotl", 0x77);
    map.insert("i32.rotr", 0x78);
    map.insert("i64.clz", 0x79);
    map.insert("i64.ctz", 0x7A);
    map.insert("i64.popcnt", 0x7B);
    map.insert("i64.add", 0x7C);
    map.insert("i64.sub", 0x7D);
    map.insert("i64.mul", 0x7E);
    map.insert("i64.div_s", 0x7F);
    map.insert("i64.div_u", 0x80);
    map.insert("i64.rem_s", 0x81);
    map.insert("i64.rem_u", 0x82);
    map.insert("i64.and", 0x83);
    map.insert("i64.or", 0x84);
    map.insert("i64.xor", 0x85);
    map.insert("i64.shl", 0x86);
    map.insert("i64.shr_s", 0x87);
    map.insert("i64.shr_u", 0x88);
    map.insert("i64.rotl", 0x89);
    map.insert("i64.rotr", 0x8A);
    map.insert("f32.abs", 0x8B);
    map.insert("f32.neg", 0x8C);
    map.insert("f32.ceil", 0x8D);
    map.insert("f32.floor", 0x8E);
    map.insert("f32.trunc", 0x8F);
    map.insert("f32.nearest", 0x90);
    map.insert("f32.sqrt", 0x91);
    map.insert("f32.add", 0x92);
    map.insert("f32.sub", 0x93);
    map.insert("f32.mul", 0x94);
    map.insert("f32.div", 0x95);
    map.insert("f32.min", 0x96);
    map.insert("f32.max", 0x97);
    map.insert("f32.copysign", 0x98);
    map.insert("f64.abs", 0x99);
    map.insert("f64.neg", 0x9A);
    map.insert("f64.ceil", 0x9B);
    map.insert("f64.floor", 0x9C);
    map.insert("f64.trunc", 0x9D);
    map.insert("f64.nearest", 0x9E);
    map.insert("f64.sqrt", 0x9F);
    map.insert("f64.add", 0xA0);
    map.insert("f64.sub", 0xA1);
    map.insert("f64.mul", 0xA2);
    map.insert("f64.div", 0xA3);
    map.insert("f64.min", 0xA4);
    map.insert("f64.max", 0xA5);
    map.insert("f64.copysign", 0xA6);
    map.insert("i32.wrap_i64", 0xA7);
    map.insert("i32.trunc_f32_s", 0xA8);
    map.insert("i32.trunc_f32_u", 0xA9);
    map.insert("i32.trunc_f64_s", 0xAA);
    map.insert("i32.trunc_f64_u", 0xAB);
    map.insert("i64.extend_i32_s", 0xAC);
    map.insert("i64.extend_i32_u", 0xAD);
    map.insert("i64.trunc_f32_s", 0xAE);
    map.insert("i64.trunc_f32_u", 0xAF);
    map.insert("i64.trunc_f64_s", 0xB0);
    map.insert("i64.trunc_f64_u", 0xB1);
    map.insert("f32.convert_i32_s", 0xB2);
    map.insert("f32.convert_i32_u", 0xB3);
    map.insert("f32.convert_i64_s", 0xB4);
    map.insert("f32.convert_i64_u", 0xB5);
    map.insert("f32.demote_f64", 0xB6);
    map.insert("f64.convert_i32_s", 0xB7);
    map.insert("f64.convert_i32_u", 0xB8);
    map.insert("f64.convert_i64_s", 0xB9);
    map.insert("f64.convert_i64_u", 0xBA);
    map.insert("f64.promote_f32", 0xBB);
    map.insert("i32.reinterpret_f32", 0xBC);
    map.insert("i64.reinterpret_f64", 0xBD);
    map.insert("f32.reinterpret_i32", 0xBE);
    map.insert("f64.reinterpret_i64", 0xBF);
    map.insert("i32.extend8_s", 0xC0);
    map.insert("i32.extend16_s", 0xC1);
    map.insert("i64.extend8_s", 0xC2);
    map.insert("i64.extend16_s", 0xC3);
    map.insert("i64.extend32_s", 0xC4);
    map.insert("ref.null", 0xD0);
    map.insert("ref.is_null", 0xD1);
    map.insert("ref.func", 0xD2);
    map.insert("ref.eq", 0xD3);
    map.insert("ref.as_non_null", 0xD4);
    map.insert("br_on_null", 0xD5);
    map.insert("br_on_non_null", 0xD6);
    map.insert("struct.new", 0xFB00);
    map.insert("struct.new_default", 0xFB01);
    map.insert("struct.get", 0xFB02);
    map.insert("struct.get_s", 0xFB03);
    map.insert("struct.get_u", 0xFB04);
    map.insert("struct.set", 0xFB05);
    map.insert("array.new", 0xFB06);
    map.insert("array.new_default", 0xFB07);
    map.insert("array.new_fixed", 0xFB08);
    map.insert("array.new_data", 0xFB09);
    map.insert("array.new_elem", 0xFB0A);
    map.insert("array.get", 0xFB0B);
    map.insert("array.get_s", 0xFB0C);
    map.insert("array.get_u", 0xFB0D);
    map.insert("array.set", 0xFB0E);
    map.insert("array.len", 0xFB0F);
    map.insert("array.fill", 0xFB10);
    map.insert("array.copy", 0xFB11);
    map.insert("array.init_data", 0xFB12);
    map.insert("array.init_elem", 0xFB13);
    map.insert("ref.test", 0xFB14);
    map.insert("ref.test.", 0xFB15);
    map.insert("ref.cast", 0xFB16);
    map.insert("ref.cast.", 0xFB17);
    map.insert("br_on_cast", 0xFB18);
    map.insert("br_on_cast_fail", 0xFB19);
    map.insert("any.convert_extern", 0xFB1A);
    map.insert("extern.convert_any", 0xFB1B);
    map.insert("ref.i31", 0xFB1C);
    map.insert("i31.get_s", 0xFB1D);
    map.insert("i31.get_u", 0xFB1E);
    map.insert("i32.trunc_sat_f32_s", 0xFC00);
    map.insert("i32.trunc_sat_f32_u", 0xFC01);
    map.insert("i32.trunc_sat_f64_s", 0xFC02);
    map.insert("i32.trunc_sat_f64_u", 0xFC03);
    map.insert("i64.trunc_sat_f32_s", 0xFC04);
    map.insert("i64.trunc_sat_f32_u", 0xFC05);
    map.insert("i64.trunc_sat_f64_s", 0xFC06);
    map.insert("i64.trunc_sat_f64_u", 0xFC07);
    map.insert("memory.init", 0xFC08);
    map.insert("data.drop", 0xFC09);
    map.insert("memory.copy", 0xFC0A);
    map.insert("memory.fill", 0xFC0B);
    map.insert("table.init", 0xFC0C);
    map.insert("elem.drop", 0xFC0D);
    map.insert("table.copy", 0xFC0E);
    map.insert("table.grow", 0xFC0F);
    map.insert("table.size", 0xFC10);
    map.insert("table.fill", 0xFC11);
    map.insert("v128.load", 0xFD00);
    map.insert("v128.load8x8_s", 0xFD01);
    map.insert("v128.load8x8_u", 0xFD02);
    map.insert("v128.load16x4_s", 0xFD03);
    map.insert("v128.load16x4_u", 0xFD04);
    map.insert("v128.load32x2_s", 0xFD05);
    map.insert("v128.load32x2_u", 0xFD06);
    map.insert("v128.load8_splat", 0xFD07);
    map.insert("v128.load16_splat", 0xFD08);
    map.insert("v128.load32_splat", 0xFD09);
    map.insert("v128.load64_splat", 0xFD0A);
    map.insert("v128.store", 0xFD0B);
    map.insert("v128.const", 0xFD0C);
    map.insert("i8x16.shuffle", 0xFD0D);
    map.insert("i8x16.swizzle", 0xFD0E);
    map.insert("i8x16.splat", 0xFD0F);
    map.insert("i16x8.splat", 0xFD10);
    map.insert("i32x4.splat", 0xFD11);
    map.insert("i64x2.splat", 0xFD12);
    map.insert("f32x4.splat", 0xFD13);
    map.insert("f64x2.splat", 0xFD14);
    map.insert("i8x16.extract_lane_s", 0xFD15);
    map.insert("i8x16.extract_lane_u", 0xFD16);
    map.insert("i8x16.replace_lane", 0xFD17);
    map.insert("i16x8.extract_lane_s", 0xFD18);
    map.insert("i16x8.extract_lane_u", 0xFD19);
    map.insert("i16x8.replace_lane", 0xFD1A);
    map.insert("i32x4.extract_lane", 0xFD1B);
    map.insert("i32x4.replace_lane", 0xFD1C);
    map.insert("i64x2.extract_lane", 0xFD1D);
    map.insert("i64x2.replace_lane", 0xFD1E);
    map.insert("f32x4.extract_lane", 0xFD1F);
    map.insert("f32x4.replace_lane", 0xFD20);
    map.insert("f64x2.extract_lane", 0xFD21);
    map.insert("f64x2.replace_lane", 0xFD22);
    map.insert("i8x16.eq", 0xFD23);
    map.insert("i8x16.ne", 0xFD24);
    map.insert("i8x16.lt_s", 0xFD25);
    map.insert("i8x16.lt_u", 0xFD26);
    map.insert("i8x16.gt_s", 0xFD27);
    map.insert("i8x16.gt_u", 0xFD28);
    map.insert("i8x16.le_s", 0xFD29);
    map.insert("i8x16.le_u", 0xFD2A);
    map.insert("i8x16.ge_s", 0xFD2B);
    map.insert("i8x16.ge_u", 0xFD2C);
    map.insert("i16x8.eq", 0xFD2D);
    map.insert("i16x8.ne", 0xFD2E);
    map.insert("i16x8.lt_s", 0xFD2F);
    map.insert("i16x8.lt_u", 0xFD30);
    map.insert("i16x8.gt_s", 0xFD31);
    map.insert("i16x8.gt_u", 0xFD32);
    map.insert("i16x8.le_s", 0xFD33);
    map.insert("i16x8.le_u", 0xFD34);
    map.insert("i16x8.ge_s", 0xFD35);
    map.insert("i16x8.ge_u", 0xFD36);
    map.insert("i32x4.eq", 0xFD37);
    map.insert("i32x4.ne", 0xFD38);
    map.insert("i32x4.lt_s", 0xFD39);
    map.insert("i32x4.lt_u", 0xFD3A);
    map.insert("i32x4.gt_s", 0xFD3B);
    map.insert("i32x4.gt_u", 0xFD3C);
    map.insert("i32x4.le_s", 0xFD3D);
    map.insert("i32x4.le_u", 0xFD3E);
    map.insert("i32x4.ge_s", 0xFD3F);
    map.insert("i32x4.ge_u", 0xFD40);
    map.insert("f32x4.eq", 0xFD41);
    map.insert("f32x4.ne", 0xFD42);
    map.insert("f32x4.lt", 0xFD43);
    map.insert("f32x4.gt", 0xFD44);
    map.insert("f32x4.le", 0xFD45);
    map.insert("f32x4.ge", 0xFD46);
    map.insert("f64x2.eq", 0xFD47);
    map.insert("f64x2.ne", 0xFD48);
    map.insert("f64x2.lt", 0xFD49);
    map.insert("f64x2.gt", 0xFD4A);
    map.insert("f64x2.le", 0xFD4B);
    map.insert("f64x2.ge", 0xFD4C);
    map.insert("v128.not", 0xFD4D);
    map.insert("v128.and", 0xFD4E);
    map.insert("v128.andnot", 0xFD4F);
    map.insert("v128.or", 0xFD50);
    map.insert("v128.xor", 0xFD51);
    map.insert("v128.bitselect", 0xFD52);
    map.insert("v128.any_true", 0xFD53);
    map.insert("v128.load8_lane", 0xFD54);
    map.insert("v128.load16_lane", 0xFD55);
    map.insert("v128.load32_lane", 0xFD56);
    map.insert("v128.load64_lane", 0xFD57);
    map.insert("v128.store8_lane", 0xFD58);
    map.insert("v128.store16_lane", 0xFD59);
    map.insert("v128.store32_lane", 0xFD5A);
    map.insert("v128.store64_lane", 0xFD5B);
    map.insert("v128.load32_zero", 0xFD5C);
    map.insert("v128.load64_zero", 0xFD5D);
    map.insert("f32x4.demote_f64x2_zero", 0xFD5E);
    map.insert("f64x2.promote_low_f32x4", 0xFD5F);
    map.insert("i8x16.abs", 0xFD60);
    map.insert("i8x16.neg", 0xFD61);
    map.insert("i8x16.popcnt", 0xFD62);
    map.insert("i8x16.all_true", 0xFD63);
    map.insert("i8x16.bitmask", 0xFD64);
    map.insert("i8x16.narrow_i16x8_s", 0xFD65);
    map.insert("i8x16.narrow_i16x8_u", 0xFD66);
    map.insert("f32x4.ceil", 0xFD67);
    map.insert("f32x4.floor", 0xFD68);
    map.insert("f32x4.trunc", 0xFD69);
    map.insert("f32x4.nearest", 0xFD6A);
    map.insert("i8x16.shl", 0xFD6B);
    map.insert("i8x16.shr_s", 0xFD6C);
    map.insert("i8x16.shr_u", 0xFD6D);
    map.insert("i8x16.add", 0xFD6E);
    map.insert("i8x16.add_sat_s", 0xFD6F);
    map.insert("i8x16.add_sat_u", 0xFD70);
    map.insert("i8x16.sub", 0xFD71);
    map.insert("i8x16.sub_sat_s", 0xFD72);
    map.insert("i8x16.sub_sat_u", 0xFD73);
    map.insert("f64x2.ceil", 0xFD74);
    map.insert("f64x2.floor", 0xFD75);
    map.insert("i8x16.min_s", 0xFD76);
    map.insert("i8x16.min_u", 0xFD77);
    map.insert("i8x16.max_s", 0xFD78);
    map.insert("i8x16.max_u", 0xFD79);
    map.insert("f64x2.trunc", 0xFD7A);
    map.insert("i8x16.avgr_u", 0xFD7B);
    map.insert("i16x8.extadd_pairwise_i8x16_s", 0xFD7C);
    map.insert("i16x8.extadd_pairwise_i8x16_u", 0xFD7D);
    map.insert("i32x4.extadd_pairwise_i16x8_s", 0xFD7E);
    map.insert("i32x4.extadd_pairwise_i16x8_u", 0xFD7F);
    map.insert("i16x8.abs", 0xFD8001);
    map.insert("i16x8.neg", 0xFD8101);
    map.insert("i16x8.q15mulr_sat_s", 0xFD8201);
    map.insert("i16x8.all_true", 0xFD8301);
    map.insert("i16x8.bitmask", 0xFD8401);
    map.insert("i16x8.narrow_i32x4_s", 0xFD8501);
    map.insert("i16x8.narrow_i32x4_u", 0xFD8601);
    map.insert("i16x8.extend_low_i8x16_s", 0xFD8701);
    map.insert("i16x8.extend_high_i8x16_s", 0xFD8801);
    map.insert("i16x8.extend_low_i8x16_u", 0xFD8901);
    map.insert("i16x8.extend_high_i8x16_u", 0xFD8A01);
    map.insert("i16x8.shl", 0xFD8B01);
    map.insert("i16x8.shr_s", 0xFD8C01);
    map.insert("i16x8.shr_u", 0xFD8D01);
    map.insert("i16x8.add", 0xFD8E01);
    map.insert("i16x8.add_sat_s", 0xFD8F01);
    map.insert("i16x8.add_sat_u", 0xFD9001);
    map.insert("i16x8.sub", 0xFD9101);
    map.insert("i16x8.sub_sat_s", 0xFD9201);
    map.insert("i16x8.sub_sat_u", 0xFD9301);
    map.insert("f64x2.nearest", 0xFD9401);
    map.insert("i16x8.mul", 0xFD9501);
    map.insert("i16x8.min_s", 0xFD9601);
    map.insert("i16x8.min_u", 0xFD9701);
    map.insert("i16x8.max_s", 0xFD9801);
    map.insert("i16x8.max_u", 0xFD9901);
    map.insert("i16x8.avgr_u", 0xFD9B01);
    map.insert("i16x8.extmul_low_i8x16_s", 0xFD9C01);
    map.insert("i16x8.extmul_high_i8x16_s", 0xFD9D01);
    map.insert("i16x8.extmul_low_i8x16_u", 0xFD9E01);
    map.insert("i16x8.extmul_high_i8x16_u", 0xFD9F01);
    map.insert("i32x4.abs", 0xFDA001);
    map.insert("i32x4.neg", 0xFDA101);
    map.insert("i32x4.all_true", 0xFDA301);
    map.insert("i32x4.bitmask", 0xFDA401);
    map.insert("i32x4.extend_low_i16x8_s", 0xFDA701);
    map.insert("i32x4.extend_high_i16x8_s", 0xFDA801);
    map.insert("i32x4.extend_low_i16x8_u", 0xFDA901);
    map.insert("i32x4.extend_high_i16x8_u", 0xFDAA01);
    map.insert("i32x4.shl", 0xFDAB01);
    map.insert("i32x4.shr_s", 0xFDAC01);
    map.insert("i32x4.shr_u", 0xFDAD01);
    map.insert("i32x4.add", 0xFDAE01);
    map.insert("i32x4.sub", 0xFDB101);
    map.insert("i32x4.mul", 0xFDB501);
    map.insert("i32x4.min_s", 0xFDB601);
    map.insert("i32x4.min_u", 0xFDB701);
    map.insert("i32x4.max_s", 0xFDB801);
    map.insert("i32x4.max_u", 0xFDB901);
    map.insert("i32x4.dot_i16x8_s", 0xFDBA01);
    map.insert("i32x4.extmul_low_i16x8_s", 0xFDBC01);
    map.insert("i32x4.extmul_high_i16x8_s", 0xFDBD01);
    map.insert("i32x4.extmul_low_i16x8_u", 0xFDBE01);
    map.insert("i32x4.extmul_high_i16x8_u", 0xFDBF01);
    map.insert("i64x2.abs", 0xFDC001);
    map.insert("i64x2.neg", 0xFDC101);
    map.insert("i64x2.all_true", 0xFDC301);
    map.insert("i64x2.bitmask", 0xFDC401);
    map.insert("i64x2.extend_low_i32x4_s", 0xFDC701);
    map.insert("i64x2.extend_high_i32x4_s", 0xFDC801);
    map.insert("i64x2.extend_low_i32x4_u", 0xFDC901);
    map.insert("i64x2.extend_high_i32x4_u", 0xFDCA01);
    map.insert("i64x2.shl", 0xFDCB01);
    map.insert("i64x2.shr_s", 0xFDCC01);
    map.insert("i64x2.shr_u", 0xFDCD01);
    map.insert("i64x2.add", 0xFDCE01);
    map.insert("i64x2.sub", 0xFDD101);
    map.insert("i64x2.mul", 0xFDD501);
    map.insert("i64x2.eq", 0xFDD601);
    map.insert("i64x2.ne", 0xFDD701);
    map.insert("i64x2.lt_s", 0xFDD801);
    map.insert("i64x2.gt_s", 0xFDD901);
    map.insert("i64x2.le_s", 0xFDDA01);
    map.insert("i64x2.ge_s", 0xFDDB01);
    map.insert("i64x2.extmul_low_i32x4_s", 0xFDDC01);
    map.insert("i64x2.extmul_high_i32x4_s", 0xFDDD01);
    map.insert("i64x2.extmul_low_i32x4_u", 0xFDDE01);
    map.insert("i64x2.extmul_high_i32x4_u", 0xFDDF01);
    map.insert("f32x4.abs", 0xFDE001);
    map.insert("f32x4.neg", 0xFDE101);
    map.insert("f32x4.sqrt", 0xFDE301);
    map.insert("f32x4.add", 0xFDE401);
    map.insert("f32x4.sub", 0xFDE501);
    map.insert("f32x4.mul", 0xFDE601);
    map.insert("f32x4.div", 0xFDE701);
    map.insert("f32x4.min", 0xFDE801);
    map.insert("f32x4.max", 0xFDE901);
    map.insert("f32x4.pmin", 0xFDEA01);
    map.insert("f32x4.pmax", 0xFDEB01);
    map.insert("f64x2.abs", 0xFDEC01);
    map.insert("f64x2.neg", 0xFDED01);
    map.insert("f64x2.sqrt", 0xFDEF01);
    map.insert("f64x2.add", 0xFDF001);
    map.insert("f64x2.sub", 0xFDF101);
    map.insert("f64x2.mul", 0xFDF201);
    map.insert("f64x2.div", 0xFDF301);
    map.insert("f64x2.min", 0xFDF401);
    map.insert("f64x2.max", 0xFDF501);
    map.insert("f64x2.pmin", 0xFDF601);
    map.insert("f64x2.pmax", 0xFDF701);
    map.insert("i32x4.trunc_sat_f32x4_s", 0xFDF801);
    map.insert("i32x4.trunc_sat_f32x4_u", 0xFDF901);
    map.insert("f32x4.convert_i32x4_s", 0xFDFA01);
    map.insert("f32x4.convert_i32x4_u", 0xFDFB01);
    map.insert("i32x4.trunc_sat_f64x2_s_zero", 0xFDFC01);
    map.insert("i32x4.trunc_sat_f64x2_u_zero", 0xFDFD01);
    map.insert("f64x2.convert_low_i32x4_s", 0xFDFE01);
    map.insert("f64x2.convert_low_i32x4_u", 0xFDFF01);
    map.insert("i8x16.relaxed_swizzle", 0xFD8002);
    map.insert("i32x4.relaxed_trunc_f32x4_s", 0xFD8102);
    map.insert("i32x4.relaxed_trunc_f32x4_u", 0xFD8202);
    map.insert("i32x4.relaxed_trunc_f64x2_s", 0xFD8302);
    map.insert("i32x4.relaxed_trunc_f64x2_u", 0xFD8402);
    map.insert("f32x4.relaxed_madd", 0xFD8502);
    map.insert("f32x4.relaxed_nmadd", 0xFD8602);
    map.insert("f64x2.relaxed_madd", 0xFD8702);
    map.insert("f64x2.relaxed_nmadd", 0xFD8802);
    map.insert("i8x16.relaxed_laneselect", 0xFD8902);
    map.insert("i16x8.relaxed_laneselect", 0xFD8A02);
    map.insert("i32x4.relaxed_laneselect", 0xFD8B02);
    map.insert("i64x2.relaxed_laneselect", 0xFD8C02);
    map.insert("f32x4.relaxed_min", 0xFD8D02);
    map.insert("f32x4.relaxed_max", 0xFD8E02);
    map.insert("f64x2.relaxed_min", 0xFD8F02);
    map.insert("f64x2.relaxed_max", 0xFD9002);
    map.insert("i16x8.relaxed_q15mulr_s", 0xFD9102);
    map.insert("i16x8.relaxed_dot_i8x16_i7x16_s", 0xFD9202);
    map.insert("i32x4.relaxed_dot_i8x16_i7x16_add_s", 0xFD9302);
    map
});
