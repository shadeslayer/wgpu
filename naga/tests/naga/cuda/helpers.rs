use naga::front::cuda::{parse_str, Options};

#[track_caller]
pub(crate) fn validate(source: &str) -> (naga::Module, naga::valid::ModuleInfo) {
    validate_with_options(source, &Options::default())
}

#[track_caller]
pub(crate) fn validate_with_options(
    source: &str,
    options: &Options,
) -> (naga::Module, naga::valid::ModuleInfo) {
    let module = parse_str(source, options).unwrap();
    let info = naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&module)
    .unwrap();
    (module, info)
}

#[track_caller]
pub(crate) fn parse_and_validate(source: &str) -> naga::Module {
    validate(source).0
}

#[cfg(all(feature = "wgsl-in", feature = "wgsl-out"))]
#[track_caller]
pub(crate) fn wgsl_roundtrip(
    module: &naga::Module,
    info: &naga::valid::ModuleInfo,
    flags: naga::back::wgsl::WriterFlags,
) -> (String, naga::Module) {
    let output = naga::back::wgsl::write_string(module, info, flags).unwrap();
    let reparsed =
        naga::front::wgsl::parse_str(&output).unwrap_or_else(|error| panic!("{error:?}\n{output}"));
    naga::valid::Validator::new(
        naga::valid::ValidationFlags::all(),
        naga::valid::Capabilities::empty(),
    )
    .validate(&reparsed)
    .unwrap();
    (output, reparsed)
}
