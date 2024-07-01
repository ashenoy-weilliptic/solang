// SPDX-License-Identifier: Apache-2.0

pub(super) mod target;
use crate::codegen::cfg::ControlFlowGraph;
use crate::emit::cfg::emit_cfg;
use crate::{
    codegen::{cfg::ASTFunction, Options},
    emit::Binary,
    sema::ast,
};
use inkwell::{
    context::Context,
    module::{Linkage, Module},
};
use soroban_sdk::xdr::{
    DepthLimitedWrite, ScEnvMetaEntry, ScSpecEntry, ScSpecFunctionInputV0, ScSpecFunctionV0,
    ScSpecTypeDef, StringM, WriteXdr,
};

pub struct WeillipticTarget;

impl WeillipticTarget {
    pub fn build<'a>(
        context: &'a Context,
        std_lib: &Module<'a>,
        contract: &'a ast::Contract,
        ns: &'a ast::Namespace,
        opt: &'a Options,
    ) -> Binary<'a> {
        let filename = ns.files[contract.loc.file_no()].file_name();
        let mut binary = Binary::new(
            context,
            ns.target,
            &contract.id.name,
            &filename,
            opt,
            std_lib,
            None,
        );

        binary
    }
}