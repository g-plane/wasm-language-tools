use crate::types_analyzer::{OperandType, ValType};
use rustc_hash::{FxBuildHasher, FxHashMap};
use std::{collections::HashMap, sync::LazyLock};

pub(crate) struct InstrMeta {
    pub bin_op: &'static str,
    pub params: Vec<OperandType>,
    pub results: Vec<OperandType>,
}

pub(crate) static INSTR_METAS: LazyLock<FxHashMap<&'static str, InstrMeta>> = LazyLock::new(|| {
    let mut map = HashMap::with_capacity_and_hasher(437, FxBuildHasher);
    map.insert(
        "unreachable",
        InstrMeta {
            bin_op: "0x00",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "nop",
        InstrMeta {
            bin_op: "0x01",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "block",
        InstrMeta {
            bin_op: "0x02",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "loop",
        InstrMeta {
            bin_op: "0x03",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "if",
        InstrMeta {
            bin_op: "0x04",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![],
        },
    );
    map.insert(
        "else",
        InstrMeta {
            bin_op: "0x05",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "end",
        InstrMeta {
            bin_op: "0x0B",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "br",
        InstrMeta {
            bin_op: "0x0C",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "br_if",
        InstrMeta {
            bin_op: "0x0D",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![],
        },
    );
    map.insert(
        "br_table",
        InstrMeta {
            bin_op: "0x0E",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![],
        },
    );
    map.insert(
        "return",
        InstrMeta {
            bin_op: "0x0F",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "call",
        InstrMeta {
            bin_op: "0x10",
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "call_indirect",
        InstrMeta {
            bin_op: "0x11",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![],
        },
    );
    map.insert(
        "drop",
        InstrMeta {
            bin_op: "0x1A",
            params: vec![OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "select",
        InstrMeta {
            bin_op: "0x1B",
            params: vec![
                OperandType::Any,
                OperandType::Any,
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "select",
        InstrMeta {
            bin_op: "0x1C",
            params: vec![
                OperandType::Any,
                OperandType::Any,
                OperandType::Val(ValType::I32),
            ],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "local.get",
        InstrMeta {
            bin_op: "0x20",
            params: vec![],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "local.set",
        InstrMeta {
            bin_op: "0x21",
            params: vec![OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "local.tee",
        InstrMeta {
            bin_op: "0x22",
            params: vec![OperandType::Any],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "global.get",
        InstrMeta {
            bin_op: "0x23",
            params: vec![],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "global.set",
        InstrMeta {
            bin_op: "0x24",
            params: vec![OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "table.get",
        InstrMeta {
            bin_op: "0x25",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "table.set",
        InstrMeta {
            bin_op: "0x26",
            params: vec![OperandType::Val(ValType::I32), OperandType::Any],
            results: vec![],
        },
    );
    map.insert(
        "i32.load",
        InstrMeta {
            bin_op: "0x28",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.load",
        InstrMeta {
            bin_op: "0x29",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.load",
        InstrMeta {
            bin_op: "0x2A",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.load",
        InstrMeta {
            bin_op: "0x2B",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.load8_s",
        InstrMeta {
            bin_op: "0x2C",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load8_u",
        InstrMeta {
            bin_op: "0x2D",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load16_s",
        InstrMeta {
            bin_op: "0x2E",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.load16_u",
        InstrMeta {
            bin_op: "0x2F",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.load8_s",
        InstrMeta {
            bin_op: "0x30",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load8_u",
        InstrMeta {
            bin_op: "0x31",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load16_s",
        InstrMeta {
            bin_op: "0x32",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load16_u",
        InstrMeta {
            bin_op: "0x33",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load32_s",
        InstrMeta {
            bin_op: "0x34",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.load32_u",
        InstrMeta {
            bin_op: "0x35",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i32.store",
        InstrMeta {
            bin_op: "0x36",
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
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "memory.grow",
        InstrMeta {
            bin_op: "0x40",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.const",
        InstrMeta {
            bin_op: "0x41",
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.const",
        InstrMeta {
            bin_op: "0x42",
            params: vec![],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.const",
        InstrMeta {
            bin_op: "0x43",
            params: vec![],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.const",
        InstrMeta {
            bin_op: "0x44",
            params: vec![],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.eqz",
        InstrMeta {
            bin_op: "0x45",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.eq",
        InstrMeta {
            bin_op: "0x46",
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
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.eq",
        InstrMeta {
            bin_op: "0x51",
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
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.ctz",
        InstrMeta {
            bin_op: "0x68",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.popcnt",
        InstrMeta {
            bin_op: "0x69",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.add",
        InstrMeta {
            bin_op: "0x6A",
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
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.ctz",
        InstrMeta {
            bin_op: "0x7A",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.popcnt",
        InstrMeta {
            bin_op: "0x7B",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.add",
        InstrMeta {
            bin_op: "0x7C",
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
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.neg",
        InstrMeta {
            bin_op: "0x8C",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.ceil",
        InstrMeta {
            bin_op: "0x8D",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.floor",
        InstrMeta {
            bin_op: "0x8E",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.trunc",
        InstrMeta {
            bin_op: "0x8F",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.nearest",
        InstrMeta {
            bin_op: "0x90",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.sqrt",
        InstrMeta {
            bin_op: "0x91",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.add",
        InstrMeta {
            bin_op: "0x92",
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
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.neg",
        InstrMeta {
            bin_op: "0x9A",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.ceil",
        InstrMeta {
            bin_op: "0x9B",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.floor",
        InstrMeta {
            bin_op: "0x9C",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.trunc",
        InstrMeta {
            bin_op: "0x9D",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.nearest",
        InstrMeta {
            bin_op: "0x9E",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.sqrt",
        InstrMeta {
            bin_op: "0x9F",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.add",
        InstrMeta {
            bin_op: "0xA0",
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
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f32_s",
        InstrMeta {
            bin_op: "0xA8",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f32_u",
        InstrMeta {
            bin_op: "0xA9",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f64_s",
        InstrMeta {
            bin_op: "0xAA",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_f64_u",
        InstrMeta {
            bin_op: "0xAB",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.extend_i32_s",
        InstrMeta {
            bin_op: "0xAC",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend_i32_u",
        InstrMeta {
            bin_op: "0xAD",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f32_s",
        InstrMeta {
            bin_op: "0xAE",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f32_u",
        InstrMeta {
            bin_op: "0xAF",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f64_s",
        InstrMeta {
            bin_op: "0xB0",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_f64_u",
        InstrMeta {
            bin_op: "0xB1",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.convert_i32_s",
        InstrMeta {
            bin_op: "0xB2",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i32_u",
        InstrMeta {
            bin_op: "0xB3",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i64_s",
        InstrMeta {
            bin_op: "0xB4",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.convert_i64_u",
        InstrMeta {
            bin_op: "0xB5",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32.demote_f64",
        InstrMeta {
            bin_op: "0xB6",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.convert_i32_s",
        InstrMeta {
            bin_op: "0xB7",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i32_u",
        InstrMeta {
            bin_op: "0xB8",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i64_s",
        InstrMeta {
            bin_op: "0xB9",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.convert_i64_u",
        InstrMeta {
            bin_op: "0xBA",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64.promote_f32",
        InstrMeta {
            bin_op: "0xBB",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.reinterpret_f32",
        InstrMeta {
            bin_op: "0xBC",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.reinterpret_f64",
        InstrMeta {
            bin_op: "0xBD",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "f32.reinterpret_i32",
        InstrMeta {
            bin_op: "0xBE",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f64.reinterpret_i64",
        InstrMeta {
            bin_op: "0xBF",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "i32.extend8_s",
        InstrMeta {
            bin_op: "0xC0",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.extend16_s",
        InstrMeta {
            bin_op: "0xC1",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.extend8_s",
        InstrMeta {
            bin_op: "0xC2",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend16_s",
        InstrMeta {
            bin_op: "0xC3",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.extend32_s",
        InstrMeta {
            bin_op: "0xC4",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "ref.null",
        InstrMeta {
            bin_op: "0xD0",
            params: vec![],
            results: vec![OperandType::Any],
        },
    );
    map.insert(
        "ref.is_null",
        InstrMeta {
            bin_op: "0xD1",
            params: vec![OperandType::Any],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "ref.func",
        InstrMeta {
            bin_op: "0xD2",
            params: vec![],
            results: vec![OperandType::Val(ValType::FuncRef)],
        },
    );
    map.insert(
        "i32.trunc_sat_f32_s",
        InstrMeta {
            bin_op: "0xFC 0x00",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f32_u",
        InstrMeta {
            bin_op: "0xFC 0x01",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f64_s",
        InstrMeta {
            bin_op: "0xFC 0x02",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32.trunc_sat_f64_u",
        InstrMeta {
            bin_op: "0xFC 0x03",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64.trunc_sat_f32_s",
        InstrMeta {
            bin_op: "0xFC 0x04",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f32_u",
        InstrMeta {
            bin_op: "0xFC 0x05",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f64_s",
        InstrMeta {
            bin_op: "0xFC 0x06",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64.trunc_sat_f64_u",
        InstrMeta {
            bin_op: "0xFC 0x07",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "memory.init",
        InstrMeta {
            bin_op: "0xFC 0x08",
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
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "memory.copy",
        InstrMeta {
            bin_op: "0xFC 0x0A",
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
            params: vec![],
            results: vec![],
        },
    );
    map.insert(
        "table.copy",
        InstrMeta {
            bin_op: "0xFC 0x0E",
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
            params: vec![OperandType::Any, OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "table.size",
        InstrMeta {
            bin_op: "0xFC 0x10",
            params: vec![],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "table.fill",
        InstrMeta {
            bin_op: "0xFC 0x11",
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
        InstrMeta {
            bin_op: "0xFD 0x00",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8x8_s",
        InstrMeta {
            bin_op: "0xFD 0x01",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8x8_u",
        InstrMeta {
            bin_op: "0xFD 0x02",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16x4_s",
        InstrMeta {
            bin_op: "0xFD 0x03",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16x4_u",
        InstrMeta {
            bin_op: "0xFD 0x04",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32x2_s",
        InstrMeta {
            bin_op: "0xFD 0x05",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32x2_u",
        InstrMeta {
            bin_op: "0xFD 0x06",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load8_splat",
        InstrMeta {
            bin_op: "0xFD 0x07",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load16_splat",
        InstrMeta {
            bin_op: "0xFD 0x08",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load32_splat",
        InstrMeta {
            bin_op: "0xFD 0x09",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_splat",
        InstrMeta {
            bin_op: "0xFD 0x0A",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.store",
        InstrMeta {
            bin_op: "0xFD 0x0B",
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
            params: vec![],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shuffle",
        InstrMeta {
            bin_op: "0xFD 0x0D",
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
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.splat",
        InstrMeta {
            bin_op: "0xFD 0x10",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.splat",
        InstrMeta {
            bin_op: "0xFD 0x11",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.splat",
        InstrMeta {
            bin_op: "0xFD 0x12",
            params: vec![OperandType::Val(ValType::I64)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.splat",
        InstrMeta {
            bin_op: "0xFD 0x13",
            params: vec![OperandType::Val(ValType::F32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.splat",
        InstrMeta {
            bin_op: "0xFD 0x14",
            params: vec![OperandType::Val(ValType::F64)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.extract_lane_s",
        InstrMeta {
            bin_op: "0xFD 0x15",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.extract_lane_u",
        InstrMeta {
            bin_op: "0xFD 0x16",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x17",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.extract_lane_u",
        InstrMeta {
            bin_op: "0xFD 0x19",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x1A",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x1C",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I64)],
        },
    );
    map.insert(
        "i64x2.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x1E",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::F32)],
        },
    );
    map.insert(
        "f32x4.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x20",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::F64)],
        },
    );
    map.insert(
        "f64x2.replace_lane",
        InstrMeta {
            bin_op: "0xFD 0x22",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.and",
        InstrMeta {
            bin_op: "0xFD 0x4E",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "v128.load8_lane",
        InstrMeta {
            bin_op: "0xFD 0x54",
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
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "v128.load64_zero",
        InstrMeta {
            bin_op: "0xFD 0x5D",
            params: vec![OperandType::Val(ValType::I32)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.demote_f64x2_zero",
        InstrMeta {
            bin_op: "0xFD 0x5E",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.promote_low_f32x4",
        InstrMeta {
            bin_op: "0xFD 0x5F",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.abs",
        InstrMeta {
            bin_op: "0xFD 0x60",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.neg",
        InstrMeta {
            bin_op: "0xFD 0x61",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.popcnt",
        InstrMeta {
            bin_op: "0xFD 0x62",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.all_true",
        InstrMeta {
            bin_op: "0xFD 0x63",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.bitmask",
        InstrMeta {
            bin_op: "0xFD 0x64",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i8x16.narrow_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0x65",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.floor",
        InstrMeta {
            bin_op: "0xFD 0x68",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.trunc",
        InstrMeta {
            bin_op: "0xFD 0x69",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.nearest",
        InstrMeta {
            bin_op: "0xFD 0x6A",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.shl",
        InstrMeta {
            bin_op: "0xFD 0x6B",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.floor",
        InstrMeta {
            bin_op: "0xFD 0x75",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.min_s",
        InstrMeta {
            bin_op: "0xFD 0x76",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i8x16.avgr_u",
        InstrMeta {
            bin_op: "0xFD 0x7B",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extadd_pairwise_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x7D",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extadd_pairwise_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0x7E",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extadd_pairwise_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0x7F",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.abs",
        InstrMeta {
            bin_op: "0xFD 0x80 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.neg",
        InstrMeta {
            bin_op: "0xFD 0x81 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.q15mulr_sat_s",
        InstrMeta {
            bin_op: "0xFD 0x82 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.bitmask",
        InstrMeta {
            bin_op: "0xFD 0x84 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i16x8.narrow_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0x85 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_high_i8x16_s",
        InstrMeta {
            bin_op: "0xFD 0x88 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_low_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x89 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.extend_high_i8x16_u",
        InstrMeta {
            bin_op: "0xFD 0x8A 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.shl",
        InstrMeta {
            bin_op: "0xFD 0x8B 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i16x8.mul",
        InstrMeta {
            bin_op: "0xFD 0x95 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.neg",
        InstrMeta {
            bin_op: "0xFD 0xA1 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.all_true",
        InstrMeta {
            bin_op: "0xFD 0xA3 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.bitmask",
        InstrMeta {
            bin_op: "0xFD 0xA4 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i32x4.extend_low_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xA7 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_high_i16x8_s",
        InstrMeta {
            bin_op: "0xFD 0xA8 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_low_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0xA9 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.extend_high_i16x8_u",
        InstrMeta {
            bin_op: "0xFD 0xAA 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.shl",
        InstrMeta {
            bin_op: "0xFD 0xAB 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.neg",
        InstrMeta {
            bin_op: "0xFD 0xC1 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.all_true",
        InstrMeta {
            bin_op: "0xFD 0xC3 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64x2.bitmask",
        InstrMeta {
            bin_op: "0xFD 0xC4 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::I32)],
        },
    );
    map.insert(
        "i64x2.extend_low_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xC7 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_high_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xC8 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_low_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xC9 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.extend_high_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xCA 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i64x2.shl",
        InstrMeta {
            bin_op: "0xFD 0xCB 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.neg",
        InstrMeta {
            bin_op: "0xFD 0xE1 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.sqrt",
        InstrMeta {
            bin_op: "0xFD 0xE3 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.add",
        InstrMeta {
            bin_op: "0xFD 0xE4 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.neg",
        InstrMeta {
            bin_op: "0xFD 0xED 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.sqrt",
        InstrMeta {
            bin_op: "0xFD 0xEF 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.add",
        InstrMeta {
            bin_op: "0xFD 0xF0 0x01",
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
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xF9 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.convert_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xFA 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f32x4.convert_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xFB 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f64x2_s_zero",
        InstrMeta {
            bin_op: "0xFD 0xFC 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "i32x4.trunc_sat_f64x2_u_zero",
        InstrMeta {
            bin_op: "0xFD 0xFD 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.convert_low_i32x4_s",
        InstrMeta {
            bin_op: "0xFD 0xFE 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map.insert(
        "f64x2.convert_low_i32x4_u",
        InstrMeta {
            bin_op: "0xFD 0xFF 0x01",
            params: vec![OperandType::Val(ValType::V128)],
            results: vec![OperandType::Val(ValType::V128)],
        },
    );
    map
});
