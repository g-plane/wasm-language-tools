pub(crate) use self::{
    def_type::{CompositeType, DefType, DefTypes, find_comp_type_by_idx, get_def_types},
    extractor::{extract_addr_type, extract_elem_ref_type, extract_global_type, extract_table_ref_type, extract_type},
    instr::{InstrSigResolverCtx, resolve_instr_sig},
    renderer::{join_types, render_block_header, render_func_header, render_header},
    resolver::{resolve_br_types, resolve_field_type, resolve_param_types},
    signature::{NamedSig, ResolvedSig, Sig},
    stack::perform_types_till,
    types::{FieldType, Fields, HeapType, OperandType, RefType, StorageType, ValType},
};

mod def_type;
mod extractor;
mod instr;
mod renderer;
mod resolver;
mod signature;
mod stack;
mod types;
