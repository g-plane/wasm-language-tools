pub(crate) use self::{
    def_type::{CompositeType, DefType, DefTypes, get_def_types, get_rec_type_groups},
    extractor::{extract_global_type, extract_type},
    renderer::{render_block_header, render_func_header},
    resolver::resolve_field_type,
    resolver::{
        resolve_array_type_with_idx, resolve_br_types, resolve_field_type_with_struct_idx,
        resolve_param_types,
    },
    signature::{ResolvedSig, get_block_sig, get_func_sig, get_type_use_sig},
    types::{
        FieldType, Fields, HeapType, OperandType, RefType, StorageType, ValType,
        operand_type_matches,
    },
};

mod def_type;
mod extractor;
mod renderer;
mod resolver;
mod signature;
mod types;
