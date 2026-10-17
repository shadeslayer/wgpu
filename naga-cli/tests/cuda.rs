use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

struct TestFiles {
    directory: PathBuf,
    input: PathBuf,
    output: PathBuf,
}

impl TestFiles {
    fn new(extension: &str, source: &str) -> Self {
        static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
        let directory = std::env::temp_dir().join(format!(
            "naga-cli-cuda-{}-{}",
            std::process::id(),
            NEXT_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&directory).unwrap();
        let input = directory.join(format!("kernel.{extension}"));
        let output = directory.join("kernel.wgsl");
        fs::write(&input, source).unwrap();
        Self {
            directory,
            input,
            output,
        }
    }

    fn translate(&self, arguments: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_naga"))
            .args(arguments)
            .arg(&self.input)
            .arg(&self.output)
            .output()
            .unwrap()
    }

    fn read_module(&self) -> naga::Module {
        let source = fs::read_to_string(&self.output).unwrap();
        let module = naga::front::wgsl::parse_str(&source).unwrap();
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::empty(),
        )
        .validate(&module)
        .unwrap();
        module
    }
}

impl Drop for TestFiles {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.directory).unwrap();
    }
}

#[test]
fn cuda_input_extension_transpiles_to_wgsl() {
    let files = TestFiles::new("cu", "__global__ void empty() {}\n");
    let result = files.translate(&["--cuda-workgroup-size", "64,1,1"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let module = files.read_module();
    assert_eq!(module.entry_points.len(), 1);
    assert_eq!(module.entry_points[0].name, "empty");
    assert_eq!(module.entry_points[0].workgroup_size, [64, 1, 1]);
}

#[test]
fn cuda_explicit_input_kind_accepts_both_names() {
    for kind in ["cuda", "cu"] {
        let files = TestFiles::new("txt", "__global__ void empty() {}\n");
        let result = files.translate(&["--input-kind", kind]);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(files.read_module().entry_points[0].workgroup_size, [1; 3]);
    }
}

#[test]
fn cuda_rejects_invalid_workgroup_dimensions() {
    for dimensions in [
        "0,1,1",
        "1,0,1",
        "1,1,0",
        "64,1",
        "64,1,1,1",
        "64,one,1",
        "4294967296,1,1",
    ] {
        let files = TestFiles::new("cu", "__global__ void empty() {}\n");
        let result = files.translate(&["--cuda-workgroup-size", dimensions]);
        assert!(!result.status.success(), "accepted {dimensions}");
        let diagnostic = String::from_utf8_lossy(&result.stderr);
        assert!(diagnostic.contains("three nonzero u32"), "{diagnostic}");
        assert!(!files.output.exists());
    }
}

#[test]
fn cuda_reports_unsupported_source_with_location() {
    let files = TestFiles::new(
        "cu",
        "__global__ void unsupported() {\n    asm(\"trap;\");\n}\n",
    );
    let result = files.translate(&[]);
    assert!(!result.status.success());
    let diagnostic = String::from_utf8_lossy(&result.stderr);
    assert!(diagnostic.contains("Could not parse CUDA"), "{diagnostic}");
    assert!(
        diagnostic.contains(&format!("{}:2:", files.input.display())),
        "{diagnostic}"
    );
    assert!(!files.output.exists());
}

#[test]
fn cuda_hlsl_grid_dimensions_require_special_constants_binding() {
    let mut files = TestFiles::new(
        "cu",
        "__global__ void grid(unsigned int *out) { out[0] = gridDim.x; }",
    );
    let wgsl = files.translate(&[]);
    assert!(
        wgsl.status.success(),
        "{}",
        String::from_utf8_lossy(&wgsl.stderr)
    );
    files.read_module();

    files.output.set_extension("hlsl");
    let hlsl = files.translate(&[]);
    assert!(!hlsl.status.success());
    let diagnostic = String::from_utf8_lossy(&hlsl.stderr);
    assert!(
        diagnostic.contains("special_constants_binding"),
        "{diagnostic}"
    );
    assert!(!files.output.exists());
}

#[test]
fn cuda_hlsl_grid_dimensions_use_configured_special_constants() {
    for (binding, declaration) in [("5,2", "register(b5, space2)"), ("0,0", "register(b0)")] {
        let mut files = TestFiles::new(
            "cu",
            "__global__ void grid(unsigned int *out) { out[0] = gridDim.x; }",
        );
        files.output.set_extension("hlsl");
        let result = files.translate(&["--hlsl-special-constants-binding", binding]);
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let hlsl = fs::read_to_string(&files.output).unwrap();
        assert!(
            hlsl.contains(&format!(
                "ConstantBuffer<NagaConstants> _NagaConstants: {declaration}"
            )),
            "{hlsl}"
        );
        assert!(
            hlsl.contains("uint3(_NagaConstants.first_vertex, _NagaConstants.first_instance, _NagaConstants.other)"),
            "{hlsl}"
        );
    }
}

#[test]
fn hlsl_rejects_invalid_special_constants_binding() {
    for binding in ["", "3", "3,0,1", "-1,0", "1,-1", "4294967296,0", "1,256"] {
        let files = TestFiles::new("cu", "__global__ void empty() {}\n");
        let result = files.translate(&["--hlsl-special-constants-binding", binding]);
        assert!(!result.status.success(), "accepted {binding}");
        let diagnostic = String::from_utf8_lossy(&result.stderr);
        assert!(diagnostic.contains("register,space"), "{diagnostic}");
        assert!(!files.output.exists());
    }
}
