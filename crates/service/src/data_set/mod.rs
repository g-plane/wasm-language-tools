pub(crate) use instr_meta::*;

mod instr_meta;

pub(crate) static VALUE_TYPES: [&str; 7] =
    ["i32", "i64", "f32", "f64", "v128", "funcref", "externref"];

pub(crate) static INSTR_NAMES: [&str; 281] = [
    "unreachable",
    "nop",
    "block",
    "loop",
    "if",
    "else",
    "end",
    "br",
    "br_if",
    "br_table",
    "return",
    "call",
    "call_indirect",
    "drop",
    "select",
    "local.get",
    "local.set",
    "local.tee",
    "global.get",
    "global.set",
    "table.get",
    "table.set",
    "i32.load",
    "i64.load",
    "f32.load",
    "f64.load",
    "i32.load8_s",
    "i32.load8_u",
    "i32.load16_s",
    "i32.load16_u",
    "i64.load8_s",
    "i64.load8_u",
    "i64.load16_s",
    "i64.load16_u",
    "i64.load32_s",
    "i64.load32_u",
    "i32.store",
    "i64.store",
    "f32.store",
    "f64.store",
    "i32.store8",
    "i32.store16",
    "i64.store8",
    "i64.store16",
    "i64.store32",
    "memory.size",
    "memory.grow",
    "i32.const",
    "i64.const",
    "f32.const",
    "f64.const",
    "i32.eqz",
    "i32.eq",
    "i32.ne",
    "i32.lt_s",
    "i32.lt_u",
    "i32.gt_s",
    "i32.gt_u",
    "i32.le_s",
    "i32.le_u",
    "i32.ge_s",
    "i32.ge_u",
    "i64.eqz",
    "i64.eq",
    "i64.ne",
    "i64.lt_s",
    "i64.lt_u",
    "i64.gt_s",
    "i64.gt_u",
    "i64.le_s",
    "i64.le_u",
    "i64.ge_s",
    "i64.ge_u",
    "f32.eq",
    "f32.ne",
    "f32.lt",
    "f32.gt",
    "f32.le",
    "f32.ge",
    "f64.eq",
    "f64.ne",
    "f64.lt",
    "f64.gt",
    "f64.le",
    "f64.ge",
    "i32.clz",
    "i32.ctz",
    "i32.popcnt",
    "i32.add",
    "i32.sub",
    "i32.mul",
    "i32.div_s",
    "i32.div_u",
    "i32.rem_s",
    "i32.rem_u",
    "i32.and",
    "i32.or",
    "i32.xor",
    "i32.shl",
    "i32.shr_s",
    "i32.shr_u",
    "i32.rotl",
    "i32.rotr",
    "i64.clz",
    "i64.ctz",
    "i64.popcnt",
    "i64.add",
    "i64.sub",
    "i64.mul",
    "i64.div_s",
    "i64.div_u",
    "i64.rem_s",
    "i64.rem_u",
    "i64.and",
    "i64.or",
    "i64.xor",
    "i64.shl",
    "i64.shr_s",
    "i64.shr_u",
    "i64.rotl",
    "i64.rotr",
    "f32.abs",
    "f32.neg",
    "f32.ceil",
    "f32.floor",
    "f32.trunc",
    "f32.nearest",
    "f32.sqrt",
    "f32.add",
    "f32.sub",
    "f32.mul",
    "f32.div",
    "f32.min",
    "f32.max",
    "f32.copysign",
    "f64.abs",
    "f64.neg",
    "f64.ceil",
    "f64.floor",
    "f64.trunc",
    "f64.nearest",
    "f64.sqrt",
    "f64.add",
    "f64.sub",
    "f64.mul",
    "f64.div",
    "f64.min",
    "f64.max",
    "f64.copysign",
    "i32.wrap_i64",
    "i32.trunc_f32_s",
    "i32.trunc_f32_u",
    "i32.trunc_f64_s",
    "i32.trunc_f64_u",
    "i64.extend_i32_s",
    "i64.extend_i32_u",
    "i64.trunc_f32_s",
    "i64.trunc_f32_u",
    "i64.trunc_f64_s",
    "i64.trunc_f64_u",
    "f32.convert_i32_s",
    "f32.convert_i32_u",
    "f32.convert_i64_s",
    "f32.convert_i64_u",
    "f32.demote_f64",
    "f64.convert_i32_s",
    "f64.convert_i32_u",
    "f64.convert_i64_s",
    "f64.convert_i64_u",
    "f64.promote_f32",
    "i32.reinterpret_f32",
    "i64.reinterpret_f64",
    "f32.reinterpret_i32",
    "f64.reinterpret_i64",
    "i32.extend8_s",
    "i32.extend16_s",
    "i64.extend8_s",
    "i64.extend16_s",
    "i64.extend32_s",
    "ref.null",
    "ref.is_null",
    "ref.func",
    "i32.trunc_sat_f32_s",
    "i32.trunc_sat_f32_u",
    "i32.trunc_sat_f64_s",
    "i32.trunc_sat_f64_u",
    "i64.trunc_sat_f32_s",
    "i64.trunc_sat_f32_u",
    "i64.trunc_sat_f64_s",
    "i64.trunc_sat_f64_u",
    "memory.init",
    "data.drop",
    "memory.copy",
    "memory.fill",
    "table.init",
    "elem.drop",
    "table.copy",
    "table.grow",
    "table.size",
    "table.fill",
    "v128.load",
    "v128.store",
    "v128.const",
    "i8x16.splat",
    "i8x16.extract_lane_s",
    "i8x16.extract_lane_u",
    "i8x16.replace_lane",
    "i16x8.splat",
    "i16x8.extract_lane_s",
    "i16x8.extract_lane_u",
    "i16x8.replace_lane",
    "i32x4.splat",
    "i32x4.extract_lane",
    "i32x4.replace_lane",
    "i64x2.splat",
    "i64x2.extract_lane",
    "i64x2.replace_lane",
    "f32x4.splat",
    "f32x4.extract_lane",
    "f32x4.replace_lane",
    "f64x2.splat",
    "f64x2.extract_lane",
    "f64x2.replace_lane",
    "i8x16.swizzle",
    "i8x16.shuffle",
    "i8x16.add",
    "i8x16.add_sat_s",
    "i8x16.add_sat_u",
    "i8x16.sub",
    "i8x16.sub_sat_s",
    "i8x16.sub_sat_u",
    "i8x16.mul",
    "i8x16.min_s",
    "i8x16.min_u",
    "i8x16.max_s",
    "i8x16.max_u",
    "i8x16.avgr_u",
    "i16x8.add",
    "i16x8.add_sat_s",
    "i16x8.add_sat_u",
    "i16x8.sub",
    "i16x8.sub_sat_s",
    "i16x8.sub_sat_u",
    "i16x8.mul",
    "i16x8.min_s",
    "i16x8.min_u",
    "i16x8.max_s",
    "i16x8.max_u",
    "i16x8.avgr_u",
    "i32x4.add",
    "i32x4.sub",
    "i32x4.mul",
    "i32x4.min_s",
    "i32x4.min_u",
    "i32x4.max_s",
    "i32x4.max_u",
    "i32x4.dot_i16x8_s",
    "i64x2.add",
    "i64x2.sub",
    "i64x2.mul",
    "f32x4.add",
    "f32x4.sub",
    "f32x4.mul",
    "f32x4.div",
    "f32x4.min",
    "f32x4.max",
    "f32x4.pmin",
    "f32x4.pmax",
    "f64x2.add",
    "f64x2.sub",
    "f64x2.mul",
    "f64x2.div",
    "f64x2.min",
    "f64x2.max",
    "f64x2.pmin",
    "f64x2.pmax",
    "i32x4.trunc_sat_f32x4_s",
    "i32x4.trunc_sat_f32x4_u",
    "f32x4.convert_i32x4_s",
    "f32x4.convert_i32x4_u",
    "v128.load8_lane",
];

pub(crate) static MODULE_FIELDS: [&str; 10] = [
    "func", "type", "import", "table", "memory", "global", "export", "start", "elem", "data",
];

pub(crate) fn get_value_type_description(value_type: &str) -> Option<&'static str> {
    match value_type {
        "i32" => Some("The type `i32` classifies 32 bit integers."),
        "i64" => Some("The type `i64` classifies 64 bit integers."),
        "f32" => Some("The type `f32` classifies 32 bit floating-point data."),
        "f64" => Some("The type `f64` classifies 64 bit floating-point data."),
        "v128" => Some("The type `v128` corresponds to a 128 bit vector of packed integer or floating-point data."),
        "funcref" => Some("The type [`funcref`](https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype) denotes the infinite union of all references to [functions](https://webassembly.github.io/spec/core/syntax/modules.html#syntax-func), regardless of their [function types](https://webassembly.github.io/spec/core/syntax/types.html#syntax-functype)."),
        "externref" => Some("The type [`externref`](https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype) denotes the infinite union of all references to objects owned by the [embedder](https://webassembly.github.io/spec/core/intro/overview.html#embedder) and that can be passed into WebAssembly under this type."),
        _ => None,
    }
}

pub(crate) static PORT_DESC: [&str; 4] = ["func", "table", "memory", "global"];
