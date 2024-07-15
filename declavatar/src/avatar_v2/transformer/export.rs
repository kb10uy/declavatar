use crate::{
    avatar_v2::{
        data::{export::ExportItem, parameter::ParameterType},
        log::Log,
        transformer::{failure, success, Compiled, FirstPassData},
    },
    decl_v2::data::export::{DeclExport, DeclExports},
    log::Logger,
};

pub fn first_pass_exports_blocks(_logger: &Logger<Log>, exports_blocks: &[DeclExports]) -> Compiled<Vec<String>> {
    let mut declared_gates = vec![];
    for decl_exports in exports_blocks {
        for decl_export in &decl_exports.exports {
            let DeclExport::Gate(gate_name) = &decl_export else {
                continue;
            };
            declared_gates.push(gate_name.clone());
        }
    }
    success(declared_gates)
}

pub fn compile_exports_blocks(
    logger: &Logger<Log>,
    first_pass: &FirstPassData,
    exports_blocks: Vec<DeclExports>,
) -> Compiled<Vec<ExportItem>> {
    let mut assets = vec![];
    for (index, decl_exports) in exports_blocks.into_iter().enumerate() {
        let logger = logger.with_context(format!("exports block {index}"));
        for decl_export in decl_exports.exports {
            let Some(asset) = compile_export(&logger, first_pass, decl_export) else {
                continue;
            };
            assets.push(asset);
        }
    }

    success(assets)
}

fn compile_export(logger: &Logger<Log>, first_pass: &FirstPassData, decl_export: DeclExport) -> Compiled<ExportItem> {
    match decl_export {
        DeclExport::Gate(name) => success(ExportItem::Gate { name }),
        DeclExport::Guard(gate, parameter) => {
            let qualified = first_pass.find_read_parameter(logger, &parameter.into(), ParameterType::BOOL_TYPE)?;
            if qualified.unique {
                logger.log(Log::GateInvalidParameter(qualified.name));
                return failure();
            }
            success(ExportItem::Guard {
                gate,
                parameter: qualified.name,
            })
        }
    }
}
