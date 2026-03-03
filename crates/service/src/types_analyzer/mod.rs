pub(crate) use self::{
    def_type::{
        CompositeType, DefType, DefTypes, find_comp_type_by_idx, get_def_types, get_rec_type_groups,
        try_deref_cont_to_func,
    },
    extractor::{extract_global_type, extract_type},
    renderer::{join_types, render_block_header, render_func_header, render_header},
    resolver::resolve_field_type,
    resolver::{
        resolve_array_type_with_idx, resolve_br_types, resolve_field_type_with_struct_idx, resolve_param_types,
    },
    signature::{ResolvedSig, Signature, get_func_sig, get_type_use_sig},
    types::{FieldType, Fields, HeapType, OperandType, RefType, StorageType, ValType},
};

mod def_type;
mod extractor;
mod renderer;
mod resolver;
mod signature;
mod types;
