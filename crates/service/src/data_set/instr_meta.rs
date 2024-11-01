use crate::types_analyzer::ValType;
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{collections::HashMap, sync::LazyLock};

pub(crate) struct InstrMeta {
    pub bin_op: &'static str,
    pub operands_count: usize,
    pub params: Vec<OperandType>,
    pub results: Vec<OperandType>,
}
#[derive(Clone, Debug)]
pub(crate) enum OperandType {
    Val(ValType),
    Generic,
}
pub(crate) static INSTR_METAS: LazyLock<FxHashMap<&'static str, InstrMeta>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity_and_hasher(424, FxBuildHasher);
    map.insert(
        "drop",
        InstrMeta {
            bin_op: "0x1A",
            operands_count: 0,
            params: vec![OperandType::Generic],
            results: vec![],
        },
    );
    map.insert(
        "select",
        InstrMeta {
            bin_op: "0x1B",
            operands_count: 0,
            params: vec![
                OperandType::Generic,
                OperandType::Generic,
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "select",
        InstrMeta {
            bin_op: "0x1C",
            operands_count: 1,
            params: vec![
                OperandType::Generic,
                OperandType::Generic,
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "local.get",
        InstrMeta {
            bin_op: "0x20",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "local.set",
        InstrMeta {
            bin_op: "0x21",
            operands_count: 1,
            params: vec![OperandType::Generic],
            results: vec![],
        },
    );
    map.insert(
        "local.tee",
        InstrMeta {
            bin_op: "0x22",
            operands_count: 1,
            params: vec![OperandType::Generic],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "global.get",
        InstrMeta {
            bin_op: "0x23",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "global.set",
        InstrMeta {
            bin_op: "0x24",
            operands_count: 1,
            params: vec![OperandType::Generic],
            results: vec![],
        },
    );
    map.insert(
        "table.get",
        InstrMeta {
            bin_op: "0x25",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "table.set",
        InstrMeta {
            bin_op: "0x26",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32), OperandType::Generic],
            results: vec![],
        },
    );
    map.insert(
        "i32.load",
        InstrMeta {
            bin_op: "0x28",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.load",
        InstrMeta {
            bin_op: "0x29",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.load",
        InstrMeta {
            bin_op: "0x2A",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.load",
        InstrMeta {
            bin_op: "0x2B",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.load8_s",
        InstrMeta {
            bin_op: "0x2C",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load8_u",
        InstrMeta {
            bin_op: "0x2D",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load16_s",
        InstrMeta {
            bin_op: "0x2E",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load16_u",
        InstrMeta {
            bin_op: "0x2F",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.load8_s",
        InstrMeta {
            bin_op: "0x30",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load8_u",
        InstrMeta {
            bin_op: "0x31",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load16_s",
        InstrMeta {
            bin_op: "0x32",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load16_u",
        InstrMeta {
            bin_op: "0x33",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load32_s",
        InstrMeta {
            bin_op: "0x34",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load32_u",
        InstrMeta {
            bin_op: "0x35",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i32.store",
        InstrMeta {
            bin_op: "0x36",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store",
        InstrMeta {
            bin_op: "0x37",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "f32.store",
        InstrMeta {
            bin_op: "0x38",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "f64.store",
        InstrMeta {
            bin_op: "0x39",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::F64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i32.store8",
        InstrMeta {
            bin_op: "0x3A",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i32.store16",
        InstrMeta {
            bin_op: "0x3B",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store8",
        InstrMeta {
            bin_op: "0x3C",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store16",
        InstrMeta {
            bin_op: "0x3D",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "i64.store32",
        InstrMeta {
            bin_op: "0x3E",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I64),
            ],
            results: vec![],
        },
    );
    map.insert(
        "memory.size",
        InstrMeta {
            bin_op: "0x3F",
            operands_count: 0,
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "memory.grow",
        InstrMeta {
            bin_op: "0x40",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.const",
        InstrMeta {
            bin_op: "0x41",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.const",
        InstrMeta {
            bin_op: "0x42",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.const",
        InstrMeta {
            bin_op: "0x43",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.const",
        InstrMeta {
            bin_op: "0x44",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.eqz",
        InstrMeta {
            bin_op: "0x45",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.eq",
        InstrMeta {
            bin_op: "0x46",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ne",
        InstrMeta {
            bin_op: "0x47",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.lt_s",
        InstrMeta {
            bin_op: "0x48",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.lt_u",
        InstrMeta {
            bin_op: "0x49",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.gt_s",
        InstrMeta {
            bin_op: "0x4A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.gt_u",
        InstrMeta {
            bin_op: "0x4B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.le_s",
        InstrMeta {
            bin_op: "0x4C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.le_u",
        InstrMeta {
            bin_op: "0x4D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ge_s",
        InstrMeta {
            bin_op: "0x4E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ge_u",
        InstrMeta {
            bin_op: "0x4F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.eqz",
        InstrMeta {
            bin_op: "0x50",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.eq",
        InstrMeta {
            bin_op: "0x51",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.ne",
        InstrMeta {
            bin_op: "0x52",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.lt_s",
        InstrMeta {
            bin_op: "0x53",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.lt_u",
        InstrMeta {
            bin_op: "0x54",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.gt_s",
        InstrMeta {
            bin_op: "0x55",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.gt_u",
        InstrMeta {
            bin_op: "0x56",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.le_s",
        InstrMeta {
            bin_op: "0x57",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.le_u",
        InstrMeta {
            bin_op: "0x58",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.ge_s",
        InstrMeta {
            bin_op: "0x59",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.ge_u",
        InstrMeta {
            bin_op: "0x5A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.eq",
        InstrMeta {
            bin_op: "0x5B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.ne",
        InstrMeta {
            bin_op: "0x5C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.lt",
        InstrMeta {
            bin_op: "0x5D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.gt",
        InstrMeta {
            bin_op: "0x5E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.le",
        InstrMeta {
            bin_op: "0x5F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f32.ge",
        InstrMeta {
            bin_op: "0x60",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.eq",
        InstrMeta {
            bin_op: "0x61",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.ne",
        InstrMeta {
            bin_op: "0x62",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.lt",
        InstrMeta {
            bin_op: "0x63",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.gt",
        InstrMeta {
            bin_op: "0x64",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.le",
        InstrMeta {
            bin_op: "0x65",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "f64.ge",
        InstrMeta {
            bin_op: "0x66",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.clz",
        InstrMeta {
            bin_op: "0x67",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ctz",
        InstrMeta {
            bin_op: "0x68",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.popcnt",
        InstrMeta {
            bin_op: "0x69",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.add",
        InstrMeta {
            bin_op: "0x6A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.sub",
        InstrMeta {
            bin_op: "0x6B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.mul",
        InstrMeta {
            bin_op: "0x6C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.div_s",
        InstrMeta {
            bin_op: "0x6D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.div_u",
        InstrMeta {
            bin_op: "0x6E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rem_s",
        InstrMeta {
            bin_op: "0x6F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rem_u",
        InstrMeta {
            bin_op: "0x70",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.and",
        InstrMeta {
            bin_op: "0x71",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.or",
        InstrMeta {
            bin_op: "0x72",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.xor",
        InstrMeta {
            bin_op: "0x73",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.shl",
        InstrMeta {
            bin_op: "0x74",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.shr_s",
        InstrMeta {
            bin_op: "0x75",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.shr_u",
        InstrMeta {
            bin_op: "0x76",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rotl",
        InstrMeta {
            bin_op: "0x77",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.rotr",
        InstrMeta {
            bin_op: "0x78",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.clz",
        InstrMeta {
            bin_op: "0x79",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.ctz",
        InstrMeta {
            bin_op: "0x7A",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.popcnt",
        InstrMeta {
            bin_op: "0x7B",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.add",
        InstrMeta {
            bin_op: "0x7C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.sub",
        InstrMeta {
            bin_op: "0x7D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.mul",
        InstrMeta {
            bin_op: "0x7E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.div_s",
        InstrMeta {
            bin_op: "0x7F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.div_u",
        InstrMeta {
            bin_op: "0x80",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rem_s",
        InstrMeta {
            bin_op: "0x81",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rem_u",
        InstrMeta {
            bin_op: "0x82",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.and",
        InstrMeta {
            bin_op: "0x83",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.or",
        InstrMeta {
            bin_op: "0x84",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.xor",
        InstrMeta {
            bin_op: "0x85",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.shl",
        InstrMeta {
            bin_op: "0x86",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.shr_s",
        InstrMeta {
            bin_op: "0x87",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.shr_u",
        InstrMeta {
            bin_op: "0x88",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rotl",
        InstrMeta {
            bin_op: "0x89",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.rotr",
        InstrMeta {
            bin_op: "0x8A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::I64),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.abs",
        InstrMeta {
            bin_op: "0x8B",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.neg",
        InstrMeta {
            bin_op: "0x8C",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.ceil",
        InstrMeta {
            bin_op: "0x8D",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.floor",
        InstrMeta {
            bin_op: "0x8E",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.trunc",
        InstrMeta {
            bin_op: "0x8F",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.nearest",
        InstrMeta {
            bin_op: "0x90",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.sqrt",
        InstrMeta {
            bin_op: "0x91",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.add",
        InstrMeta {
            bin_op: "0x92",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.sub",
        InstrMeta {
            bin_op: "0x93",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.mul",
        InstrMeta {
            bin_op: "0x94",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.div",
        InstrMeta {
            bin_op: "0x95",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.min",
        InstrMeta {
            bin_op: "0x96",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.max",
        InstrMeta {
            bin_op: "0x97",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.copysign",
        InstrMeta {
            bin_op: "0x98",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F32),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.abs",
        InstrMeta {
            bin_op: "0x99",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.neg",
        InstrMeta {
            bin_op: "0x9A",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.ceil",
        InstrMeta {
            bin_op: "0x9B",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.floor",
        InstrMeta {
            bin_op: "0x9C",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.trunc",
        InstrMeta {
            bin_op: "0x9D",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.nearest",
        InstrMeta {
            bin_op: "0x9E",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.sqrt",
        InstrMeta {
            bin_op: "0x9F",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.add",
        InstrMeta {
            bin_op: "0xA0",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.sub",
        InstrMeta {
            bin_op: "0xA1",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.mul",
        InstrMeta {
            bin_op: "0xA2",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.div",
        InstrMeta {
            bin_op: "0xA3",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.min",
        InstrMeta {
            bin_op: "0xA4",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.max",
        InstrMeta {
            bin_op: "0xA5",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.copysign",
        InstrMeta {
            bin_op: "0xA6",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::F64),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.wrap_i64",
        InstrMeta {
            bin_op: "0xA7",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f32_s",
        InstrMeta {
            bin_op: "0xA8",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f32_u",
        InstrMeta {
            bin_op: "0xA9",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f64_s",
        InstrMeta {
            bin_op: "0xAA",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f64_u",
        InstrMeta {
            bin_op: "0xAB",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.extend_i32_s",
        InstrMeta {
            bin_op: "0xAC",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend_i32_u",
        InstrMeta {
            bin_op: "0xAD",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f32_s",
        InstrMeta {
            bin_op: "0xAE",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f32_u",
        InstrMeta {
            bin_op: "0xAF",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f64_s",
        InstrMeta {
            bin_op: "0xB0",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f64_u",
        InstrMeta {
            bin_op: "0xB1",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.convert_i32_s",
        InstrMeta {
            bin_op: "0xB2",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i32_u",
        InstrMeta {
            bin_op: "0xB3",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i64_s",
        InstrMeta {
            bin_op: "0xB4",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i64_u",
        InstrMeta {
            bin_op: "0xB5",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.demote_f64",
        InstrMeta {
            bin_op: "0xB6",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.convert_i32_s",
        InstrMeta {
            bin_op: "0xB7",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i32_u",
        InstrMeta {
            bin_op: "0xB8",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i64_s",
        InstrMeta {
            bin_op: "0xB9",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i64_u",
        InstrMeta {
            bin_op: "0xBA",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.promote_f32",
        InstrMeta {
            bin_op: "0xBB",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.reinterpret_f32",
        InstrMeta {
            bin_op: "0xBC",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.reinterpret_f64",
        InstrMeta {
            bin_op: "0xBD",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.reinterpret_i32",
        InstrMeta {
            bin_op: "0xBE",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.reinterpret_i64",
        InstrMeta {
            bin_op: "0xBF",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.extend8_s",
        InstrMeta {
            bin_op: "0xC0",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.extend16_s",
        InstrMeta {
            bin_op: "0xC1",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.extend8_s",
        InstrMeta {
            bin_op: "0xC2",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend16_s",
        InstrMeta {
            bin_op: "0xC3",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend32_s",
        InstrMeta {
            bin_op: "0xC4",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "ref.null",
        InstrMeta {
            bin_op: "0xD0",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Generic],
        },
    );
    map.insert(
        "ref.is_null",
        InstrMeta {
            bin_op: "0xD1",
            operands_count: 0,
            params: vec![OperandType::Generic],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "ref.func",
        InstrMeta {
            bin_op: "0xD2",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::FuncRef)],
        },
    );
    map.insert(
        "i32.trunc_sat_f32_s",
        InstrMeta {
            bin_op: "0xFC 0x00",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f32_u",
        InstrMeta {
            bin_op: "0xFC 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f64_s",
        InstrMeta {
            bin_op: "0xFC 0x02",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f64_u",
        InstrMeta {
            bin_op: "0xFC 0x03",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.trunc_sat_f32_s",
        InstrMeta {
            bin_op: "0xFC 0x04",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f32_u",
        InstrMeta {
            bin_op: "0xFC 0x05",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f64_s",
        InstrMeta {
            bin_op: "0xFC 0x06",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f64_u",
        InstrMeta {
            bin_op: "0xFC 0x07",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "memory.init",
        InstrMeta {
            bin_op: "0xFC 0x08",
            operands_count: 1,
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
        InstrMeta {
            bin_op: "0xFC 0x09",
            operands_count: 1,
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "memory.copy",
        InstrMeta {
            bin_op: "0xFC 0x0A",
            operands_count: 0,
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
        InstrMeta {
            bin_op: "0xFC 0x0B",
            operands_count: 0,
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
        InstrMeta {
            bin_op: "0xFC 0x0C",
            operands_count: 2,
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
        InstrMeta {
            bin_op: "0xFC 0x0D",
            operands_count: 1,
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "table.copy",
        InstrMeta {
            bin_op: "0xFC 0x0E",
            operands_count: 2,
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
        InstrMeta {
            bin_op: "0xFC 0x0F",
            operands_count: 1,
            params: vec![OperandType::Generic, OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "table.size",
        InstrMeta {
            bin_op: "0xFC 0x10",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "table.fill",
        InstrMeta {
            bin_op: "0xFC 0x11",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Generic,
                OperandType::Val(ValType::I32),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.load",
        InstrMeta {
            bin_op: "0xFD 0x00",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8x8_s",
        InstrMeta {
            bin_op: "0xFD 0x01",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8x8_u",
        InstrMeta {
            bin_op: "0xFD 0x02",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16x4_s",
        InstrMeta {
            bin_op: "0xFD 0x03",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16x4_u",
        InstrMeta {
            bin_op: "0xFD 0x04",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32x2_s",
        InstrMeta {
            bin_op: "0xFD 0x05",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32x2_u",
        InstrMeta {
            bin_op: "0xFD 0x06",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8_splat",
        InstrMeta {
            bin_op: "0xFD 0x07",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16_splat",
        InstrMeta {
            bin_op: "0xFD 0x08",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32_splat",
        InstrMeta {
            bin_op: "0xFD 0x09",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_splat",
        InstrMeta {
            bin_op: "0xFD 0x0A",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.store",
        InstrMeta {
            bin_op: "0xFD 0x0B",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.const",
        InstrMeta {
            bin_op: "0xFD 0x0C",
            operands_count: 1,
            params: vec![],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shuffle",
        InstrMeta {
            bin_op: "0xFD 0x0D",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.swizzle",
        InstrMeta {
            bin_op: "0xFD 0x0E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.splat",
        InstrMeta {
            bin_op: "0xFD 0x0F",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.splat",
        InstrMeta {
            bin_op: "0xFD 0x10",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.splat",
        InstrMeta {
            bin_op: "0xFD 0x11",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.splat",
        InstrMeta {
            bin_op: "0xFD 0x12",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.splat",
        InstrMeta {
            bin_op: "0xFD 0x13",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.splat",
        InstrMeta {
            bin_op: "0xFD 0x14",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.extract_lane_s",
        InstrMeta {
            bin_op: "0xFD 0x15",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.extract_lane_u",
        InstrMeta {
            bin_op: "0xFD 0x16",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x17",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extract_lane_s",
        InstrMeta {
            bin_op: "0xFD 0x18",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.extract_lane_u",
        InstrMeta {
            bin_op: "0xFD 0x19",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x1A",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extract_lane",
        InstrMeta {
            bin_op: "0xFD 0x1B",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x1C",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extract_lane",
        InstrMeta {
            bin_op: "0xFD 0x1D",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64x2.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x1E",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I64),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.extract_lane",
        InstrMeta {
            bin_op: "0xFD 0x1F",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32x4.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x20",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::F32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.extract_lane",
        InstrMeta {
            bin_op: "0xFD 0x21",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64x2.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x22",
            operands_count: 1,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::F64),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.eq",
        InstrMeta {
            bin_op: "0xFD 0x23",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.ne",
        InstrMeta {
            bin_op: "0xFD 0x24",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.lt_s",
        InstrMeta {
            bin_op: "0xFD 0x25",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.lt_u",
        InstrMeta {
            bin_op: "0xFD 0x26",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.gt_s",
        InstrMeta {
            bin_op: "0xFD 0x27",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.gt_u",
        InstrMeta {
            bin_op: "0xFD 0x28",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.le_s",
        InstrMeta {
            bin_op: "0xFD 0x29",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.le_u",
        InstrMeta {
            bin_op: "0xFD 0x2A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.ge_s",
        InstrMeta {
            bin_op: "0xFD 0x2B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.ge_u",
        InstrMeta {
            bin_op: "0xFD 0x2C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.eq",
        InstrMeta {
            bin_op: "0xFD 0x2D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.ne",
        InstrMeta {
            bin_op: "0xFD 0x2E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.lt_s",
        InstrMeta {
            bin_op: "0xFD 0x2F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.lt_u",
        InstrMeta {
            bin_op: "0xFD 0x30",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.gt_s",
        InstrMeta {
            bin_op: "0xFD 0x31",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.gt_u",
        InstrMeta {
            bin_op: "0xFD 0x32",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.le_s",
        InstrMeta {
            bin_op: "0xFD 0x33",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.le_u",
        InstrMeta {
            bin_op: "0xFD 0x34",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.ge_s",
        InstrMeta {
            bin_op: "0xFD 0x35",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.ge_u",
        InstrMeta {
            bin_op: "0xFD 0x36",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.eq",
        InstrMeta {
            bin_op: "0xFD 0x37",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.ne",
        InstrMeta {
            bin_op: "0xFD 0x38",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.lt_s",
        InstrMeta {
            bin_op: "0xFD 0x39",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.lt_u",
        InstrMeta {
            bin_op: "0xFD 0x3A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.gt_s",
        InstrMeta {
            bin_op: "0xFD 0x3B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.gt_u",
        InstrMeta {
            bin_op: "0xFD 0x3C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.le_s",
        InstrMeta {
            bin_op: "0xFD 0x3D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.le_u",
        InstrMeta {
            bin_op: "0xFD 0x3E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.ge_s",
        InstrMeta {
            bin_op: "0xFD 0x3F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.ge_u",
        InstrMeta {
            bin_op: "0xFD 0x40",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.eq",
        InstrMeta {
            bin_op: "0xFD 0x41",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.ne",
        InstrMeta {
            bin_op: "0xFD 0x42",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.lt",
        InstrMeta {
            bin_op: "0xFD 0x43",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.gt",
        InstrMeta {
            bin_op: "0xFD 0x44",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.le",
        InstrMeta {
            bin_op: "0xFD 0x45",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.ge",
        InstrMeta {
            bin_op: "0xFD 0x46",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.eq",
        InstrMeta {
            bin_op: "0xFD 0x47",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.ne",
        InstrMeta {
            bin_op: "0xFD 0x48",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.lt",
        InstrMeta {
            bin_op: "0xFD 0x49",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.gt",
        InstrMeta {
            bin_op: "0xFD 0x4A",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.le",
        InstrMeta {
            bin_op: "0xFD 0x4B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.ge",
        InstrMeta {
            bin_op: "0xFD 0x4C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.not",
        InstrMeta {
            bin_op: "0xFD 0x4D",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.and",
        InstrMeta {
            bin_op: "0xFD 0x4E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.andnot",
        InstrMeta {
            bin_op: "0xFD 0x4F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.or",
        InstrMeta {
            bin_op: "0xFD 0x50",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.xor",
        InstrMeta {
            bin_op: "0xFD 0x51",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.bitselect",
        InstrMeta {
            bin_op: "0xFD 0x52",
            operands_count: 0,
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
        InstrMeta {
            bin_op: "0xFD 0x53",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "v128.load8_lane",
        InstrMeta {
            bin_op: "0xFD 0x54",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16_lane",
        InstrMeta {
            bin_op: "0xFD 0x55",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32_lane",
        InstrMeta {
            bin_op: "0xFD 0x56",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_lane",
        InstrMeta {
            bin_op: "0xFD 0x57",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.store8_lane",
        InstrMeta {
            bin_op: "0xFD 0x58",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.store16_lane",
        InstrMeta {
            bin_op: "0xFD 0x59",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.store32_lane",
        InstrMeta {
            bin_op: "0xFD 0x5A",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.store64_lane",
        InstrMeta {
            bin_op: "0xFD 0x5B",
            operands_count: 2,
            params: vec![
                OperandType::Val(ValType::I32),
                OperandType::Val(ValType::V128),
            ],
            results: vec![],
        },
    );
    map.insert(
        "v128.load32_zero",
        InstrMeta {
            bin_op: "0xFD 0x5C",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_zero",
        InstrMeta {
            bin_op: "0xFD 0x5D",
            operands_count: 1,
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.demote_f64x2_zero",
        InstrMeta {
            bin_op: "0xFD 0x5E",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.promote_low_f32x4",
        InstrMeta {
            bin_op: "0xFD 0x5F",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.abs",
        InstrMeta {
            bin_op: "0xFD 0x60",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.neg",
        InstrMeta {
            bin_op: "0xFD 0x61",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.popcnt",
        InstrMeta {
            bin_op: "0xFD 0x62",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.all_true",
        InstrMeta {
            bin_op: "0xFD 0x63",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.bitmask",
        InstrMeta {
            bin_op: "0xFD 0x64",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.narrow_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0x65",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.narrow_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0x66",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.ceil",
        InstrMeta {
            bin_op: "0xFD 0x67",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.floor",
        InstrMeta {
            bin_op: "0xFD 0x68",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.trunc",
        InstrMeta {
            bin_op: "0xFD 0x69",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.nearest",
        InstrMeta {
            bin_op: "0xFD 0x6A",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shl",
        InstrMeta {
            bin_op: "0xFD 0x6B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shr_s",
        InstrMeta {
            bin_op: "0xFD 0x6C",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shr_u",
        InstrMeta {
            bin_op: "0xFD 0x6D",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.add",
        InstrMeta {
            bin_op: "0xFD 0x6E",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.add_sat_s",
        InstrMeta {
            bin_op: "0xFD 0x6F",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.add_sat_u",
        InstrMeta {
            bin_op: "0xFD 0x70",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.sub",
        InstrMeta {
            bin_op: "0xFD 0x71",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.sub_sat_s",
        InstrMeta {
            bin_op: "0xFD 0x72",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.sub_sat_u",
        InstrMeta {
            bin_op: "0xFD 0x73",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.ceil",
        InstrMeta {
            bin_op: "0xFD 0x74",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.floor",
        InstrMeta {
            bin_op: "0xFD 0x75",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.min_s",
        InstrMeta {
            bin_op: "0xFD 0x76",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.min_u",
        InstrMeta {
            bin_op: "0xFD 0x77",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.max_s",
        InstrMeta {
            bin_op: "0xFD 0x78",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.max_u",
        InstrMeta {
            bin_op: "0xFD 0x79",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.trunc",
        InstrMeta {
            bin_op: "0xFD 0x7A",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.avgr_u",
        InstrMeta {
            bin_op: "0xFD 0x7B",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extadd_pairwise_i8x16_s",
        InstrMeta {
            bin_op: "0xFD 0x7C",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extadd_pairwise_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x7D",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extadd_pairwise_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0x7E",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extadd_pairwise_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0x7F",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.abs",
        InstrMeta {
            bin_op: "0xFD 0x80 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.neg",
        InstrMeta {
            bin_op: "0xFD 0x81 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.q15mulr_sat_s",
        InstrMeta {
            bin_op: "0xFD 0x82 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.all_true",
        InstrMeta {
            bin_op: "0xFD 0x83 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.bitmask",
        InstrMeta {
            bin_op: "0xFD 0x84 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.narrow_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0x85 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.narrow_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0x86 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_low_i8x16_s",
        InstrMeta {
            bin_op: "0xFD 0x87 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_high_i8x16_s",
        InstrMeta {
            bin_op: "0xFD 0x88 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_low_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x89 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_high_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x8A 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shl",
        InstrMeta {
            bin_op: "0xFD 0x8B 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shr_s",
        InstrMeta {
            bin_op: "0xFD 0x8C 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shr_u",
        InstrMeta {
            bin_op: "0xFD 0x8D 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.add",
        InstrMeta {
            bin_op: "0xFD 0x8E 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.add_sat_s",
        InstrMeta {
            bin_op: "0xFD 0x8F 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.add_sat_u",
        InstrMeta {
            bin_op: "0xFD 0x90 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.sub",
        InstrMeta {
            bin_op: "0xFD 0x91 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.sub_sat_s",
        InstrMeta {
            bin_op: "0xFD 0x92 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.sub_sat_u",
        InstrMeta {
            bin_op: "0xFD 0x93 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.nearest",
        InstrMeta {
            bin_op: "0xFD 0x94 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.mul",
        InstrMeta {
            bin_op: "0xFD 0x95 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.min_s",
        InstrMeta {
            bin_op: "0xFD 0x96 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.min_u",
        InstrMeta {
            bin_op: "0xFD 0x97 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.max_s",
        InstrMeta {
            bin_op: "0xFD 0x98 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.max_u",
        InstrMeta {
            bin_op: "0xFD 0x99 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.avgr_u",
        InstrMeta {
            bin_op: "0xFD 0x9B 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_low_i8x16_s",
        InstrMeta {
            bin_op: "0xFD 0x9C 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_high_i8x16_s",
        InstrMeta {
            bin_op: "0xFD 0x9D 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_low_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x9E 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extmul_high_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x9F 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.abs",
        InstrMeta {
            bin_op: "0xFD 0xA0 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.neg",
        InstrMeta {
            bin_op: "0xFD 0xA1 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.all_true",
        InstrMeta {
            bin_op: "0xFD 0xA3 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.bitmask",
        InstrMeta {
            bin_op: "0xFD 0xA4 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.extend_low_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xA7 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_high_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xA8 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_low_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0xA9 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_high_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0xAA 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shl",
        InstrMeta {
            bin_op: "0xFD 0xAB 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shr_s",
        InstrMeta {
            bin_op: "0xFD 0xAC 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shr_u",
        InstrMeta {
            bin_op: "0xFD 0xAD 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.add",
        InstrMeta {
            bin_op: "0xFD 0xAE 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.sub",
        InstrMeta {
            bin_op: "0xFD 0xB1 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.mul",
        InstrMeta {
            bin_op: "0xFD 0xB5 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.min_s",
        InstrMeta {
            bin_op: "0xFD 0xB6 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.min_u",
        InstrMeta {
            bin_op: "0xFD 0xB7 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.max_s",
        InstrMeta {
            bin_op: "0xFD 0xB8 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.max_u",
        InstrMeta {
            bin_op: "0xFD 0xB9 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.dot_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xBA 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_low_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xBC 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_high_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xBD 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_low_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0xBE 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extmul_high_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0xBF 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.abs",
        InstrMeta {
            bin_op: "0xFD 0xC0 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.neg",
        InstrMeta {
            bin_op: "0xFD 0xC1 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.all_true",
        InstrMeta {
            bin_op: "0xFD 0xC3 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64x2.bitmask",
        InstrMeta {
            bin_op: "0xFD 0xC4 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64x2.extend_low_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xC7 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_high_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xC8 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_low_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xC9 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_high_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xCA 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shl",
        InstrMeta {
            bin_op: "0xFD 0xCB 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shr_s",
        InstrMeta {
            bin_op: "0xFD 0xCC 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shr_u",
        InstrMeta {
            bin_op: "0xFD 0xCD 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.add",
        InstrMeta {
            bin_op: "0xFD 0xCE 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.sub",
        InstrMeta {
            bin_op: "0xFD 0xD1 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.mul",
        InstrMeta {
            bin_op: "0xFD 0xD5 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.eq",
        InstrMeta {
            bin_op: "0xFD 0xD6 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.ne",
        InstrMeta {
            bin_op: "0xFD 0xD7 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.lt_s",
        InstrMeta {
            bin_op: "0xFD 0xD8 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.gt_s",
        InstrMeta {
            bin_op: "0xFD 0xD9 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.le_s",
        InstrMeta {
            bin_op: "0xFD 0xDA 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.ge_s",
        InstrMeta {
            bin_op: "0xFD 0xDB 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_low_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xDC 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_high_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xDD 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_low_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xDE 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extmul_high_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xDF 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.abs",
        InstrMeta {
            bin_op: "0xFD 0xE0 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.neg",
        InstrMeta {
            bin_op: "0xFD 0xE1 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.sqrt",
        InstrMeta {
            bin_op: "0xFD 0xE3 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.add",
        InstrMeta {
            bin_op: "0xFD 0xE4 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.sub",
        InstrMeta {
            bin_op: "0xFD 0xE5 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.mul",
        InstrMeta {
            bin_op: "0xFD 0xE6 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.div",
        InstrMeta {
            bin_op: "0xFD 0xE7 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.min",
        InstrMeta {
            bin_op: "0xFD 0xE8 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.max",
        InstrMeta {
            bin_op: "0xFD 0xE9 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.pmin",
        InstrMeta {
            bin_op: "0xFD 0xEA 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.pmax",
        InstrMeta {
            bin_op: "0xFD 0xEB 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.abs",
        InstrMeta {
            bin_op: "0xFD 0xEC 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.neg",
        InstrMeta {
            bin_op: "0xFD 0xED 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.sqrt",
        InstrMeta {
            bin_op: "0xFD 0xEF 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.add",
        InstrMeta {
            bin_op: "0xFD 0xF0 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.sub",
        InstrMeta {
            bin_op: "0xFD 0xF1 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.mul",
        InstrMeta {
            bin_op: "0xFD 0xF2 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.div",
        InstrMeta {
            bin_op: "0xFD 0xF3 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.min",
        InstrMeta {
            bin_op: "0xFD 0xF4 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.max",
        InstrMeta {
            bin_op: "0xFD 0xF5 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.pmin",
        InstrMeta {
            bin_op: "0xFD 0xF6 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.pmax",
        InstrMeta {
            bin_op: "0xFD 0xF7 0x01",
            operands_count: 0,
            params: vec![
                OperandType::Val(ValType::V128),
                OperandType::Val(ValType::V128),
            ],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xF8 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xF9 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.convert_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xFA 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.convert_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xFB 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f64x2_s_zero",
        InstrMeta {
            bin_op: "0xFD 0xFC 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f64x2_u_zero",
        InstrMeta {
            bin_op: "0xFD 0xFD 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.convert_low_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xFE 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.convert_low_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xFF 0x01",
            operands_count: 0,
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map
});
