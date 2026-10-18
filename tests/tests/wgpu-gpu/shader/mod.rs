//! Infrastructure for testing particular behavior of shaders across platforms.
//!
//! The tests take the form of a input buffer filled with u32 data. A compute
//! shader is run on the input buffer which generates an output buffer. This
//! buffer is then read and compared to a given output.

use std::{borrow::Cow, collections::HashSet, fmt::Debug};

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    Backends, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry,
    BindingType, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor,
    ComputePipelineDescriptor, MapMode, PipelineLayoutDescriptor, PollType, ShaderModuleDescriptor,
    ShaderSource, ShaderStages,
};

use wgpu_test::{GpuTestInitializer, TestingContext};

pub mod array_size_overrides;
pub mod compilation_messages;
pub mod cuda;
pub mod data_builtins;
pub mod module_label;
pub mod module_source;
pub mod numeric_builtins;
pub mod struct_layout;
pub mod workgroup_size_overrides;
pub mod zero_init_workgroup_mem;

pub fn all_tests(tests: &mut Vec<GpuTestInitializer>) {
    array_size_overrides::all_tests(tests);
    compilation_messages::all_tests(tests);
    cuda::all_tests(tests);
    data_builtins::all_tests(tests);
    module_label::all_tests(tests);
    module_source::all_tests(tests);
    numeric_builtins::all_tests(tests);
    struct_layout::all_tests(tests);
    workgroup_size_overrides::all_tests(tests);
    zero_init_workgroup_mem::all_tests(tests);
}

#[derive(Clone, Copy, PartialEq)]
enum InputStorageType {
    Uniform,
    Storage,
    Immediate,
}

impl InputStorageType {
    fn as_str(&self) -> &'static str {
        match self {
            InputStorageType::Uniform => "uniform",
            InputStorageType::Storage => "storage",
            InputStorageType::Immediate => "immediate",
        }
    }
}

struct ShaderBuffer {
    binding: u32,
    ty: wgpu::BufferBindingType,
    contents: Vec<u8>,
}

impl ShaderBuffer {
    fn new<T: bytemuck::Pod>(binding: u32, ty: wgpu::BufferBindingType, initial: &[T]) -> Self {
        Self {
            binding,
            ty,
            contents: bytemuck::cast_slice(initial).to_vec(),
        }
    }
}

struct ModuleBuffers {
    buffers: Vec<ShaderBuffer>,
    output_binding: u32,
    workgroups: [u32; 3],
}

/// Describes a single test of a shader.
struct ShaderTest {
    /// Human readable name
    name: String,
    module_source: Option<String>,
    module_buffers: Option<ModuleBuffers>,
    entry_point: String,
    /// Header text. This is arbitrary code injected at the top of the shader. Replaces {{header}}
    header: String,
    /// This text will be the body of the `Input` struct. Replaces "{{input_members}}"
    /// in the shader_test shader.
    custom_struct_members: String,
    /// This text will be the body of the compute shader. Replaces "{{body}}"
    /// in the shader_test shader.
    body: String,
    /// This text will be the input type of the compute shader. Replaces "{{input_type}}".
    ///
    /// Defaults to "CustomStruct"
    input_type: String,
    /// This text will be the output type of the compute shader. Replaces "{{output_type}}".
    ///
    /// Defaults to "array<u32>".
    output_type: String,
    /// List of values will be written to the input buffer.
    input_values: Vec<u32>,
    /// List of lists of valid expected outputs from the shader.
    output_values: Vec<Vec<u32>>,
    /// Function which compares the output values to the resulting values and
    /// prints a message on failure.
    ///
    /// Defaults [`Self::default_comparison_function`].
    output_comparison_fn: fn(&str, &[u32], &[Vec<u32>]) -> bool,
    /// Value to pre-initialize the output buffer to. Often u32::MAX so
    /// that writing a 0 looks different than not writing a value at all.
    ///
    /// Defaults to u32::MAX.
    output_initialization: u32,
    output_initial_values: Vec<u32>,
    /// Which backends this test will fail on. If the test passes on this
    /// backend when it shouldn't, an assert will be raised.
    ///
    /// Defaults to Backends::empty().
    failures: Backends,
}
impl ShaderTest {
    fn default_comparison_function<O: bytemuck::Pod + Debug + PartialEq>(
        test_name: &str,
        actual_values: &[u32],
        expected_values: &[Vec<u32>],
    ) -> bool {
        let cast_actual = bytemuck::cast_slice::<u32, O>(actual_values);

        // When printing the error message, we want to trim `cast_actual` to the length
        // of the longest set of expected values. This tracks that value.
        let mut max_relevant_value_count = 0;

        for expected in expected_values {
            let cast_expected = bytemuck::cast_slice::<u32, O>(expected);

            // We shorten the actual to the length of the expected.
            if &cast_actual[0..cast_expected.len()] == cast_expected {
                return true;
            }

            max_relevant_value_count = max_relevant_value_count.max(cast_expected.len());
        }

        // We haven't found a match, lets print an error.

        eprint!(
            "Inner test failure. Actual {:?}. Expected",
            &cast_actual[0..max_relevant_value_count]
        );

        if expected_values.len() != 1 {
            eprint!(" one of: ");
        } else {
            eprint!(": ");
        }

        for (idx, expected) in expected_values.iter().enumerate() {
            let cast_expected = bytemuck::cast_slice::<u32, O>(expected);
            eprint!("{cast_expected:?}");
            if idx + 1 != expected_values.len() {
                eprint!(" ");
            }
        }

        eprintln!(". Test {test_name}");

        false
    }

    fn new<I: bytemuck::Pod, O: bytemuck::Pod + Debug + PartialEq>(
        name: String,
        custom_struct_members: String,
        body: String,
        input_values: &[I],
        output_values: &[O],
    ) -> Self {
        Self {
            name,
            module_source: None,
            module_buffers: None,
            entry_point: String::from("cs_main"),
            header: String::new(),
            custom_struct_members,
            body,
            input_type: String::from("CustomStruct"),
            output_type: String::from("array<u32>"),
            input_values: bytemuck::pod_collect_to_vec(input_values),
            output_values: vec![bytemuck::pod_collect_to_vec(output_values)],
            output_comparison_fn: Self::default_comparison_function::<O>,
            output_initialization: u32::MAX,
            output_initial_values: Vec::new(),
            failures: Backends::empty(),
        }
    }

    fn module_source(mut self, source: String, entry_point: String) -> Self {
        self.module_source = Some(source);
        self.entry_point = entry_point;

        self
    }

    fn module_buffers(
        mut self,
        buffers: Vec<ShaderBuffer>,
        output_binding: u32,
        workgroups: [u32; 3],
    ) -> Self {
        self.module_buffers = Some(ModuleBuffers {
            buffers,
            output_binding,
            workgroups,
        });

        self
    }

    fn output_initial_values<O: bytemuck::Pod>(mut self, values: &[O]) -> Self {
        self.output_initial_values = bytemuck::pod_collect_to_vec(values);

        self
    }

    fn buffer_sizes_valid(&self) -> bool {
        if let Some(module) = &self.module_buffers {
            if self.module_source.is_none()
                || !self.output_initial_values.is_empty()
                || module.workgroups.contains(&0)
            {
                return false;
            }
            let mut bindings = HashSet::new();
            if !module.buffers.iter().all(|buffer| {
                !buffer.contents.is_empty()
                    && buffer.contents.len().is_multiple_of(4)
                    && bindings.insert(buffer.binding)
            }) {
                return false;
            }
            let Some(output) = module
                .buffers
                .iter()
                .find(|buffer| buffer.binding == module.output_binding)
            else {
                return false;
            };
            return output.ty == wgpu::BufferBindingType::Storage { read_only: false }
                && self
                    .output_values
                    .iter()
                    .all(|values| values.len() <= output.contents.len() / 4);
        }
        let max_words = MAX_BUFFER_SIZE as usize / 4;
        self.input_values.len() <= max_words
            && self
                .output_values
                .iter()
                .all(|values| values.len() <= max_words)
            && self.output_initial_values.len() <= max_words
    }

    fn header(mut self, header: String) -> Self {
        self.header = header;

        self
    }

    fn output_type(mut self, output_type: String) -> Self {
        self.output_type = output_type;

        self
    }

    /// Add another set of possible outputs. If any of the given
    /// output values are seen it's considered a success (i.e. this is OR, not AND).
    ///
    /// Assumes that this type O is the same as the O provided to new.
    fn extra_output_values<O: bytemuck::Pod + Debug + PartialEq>(
        mut self,
        output_values: &[O],
    ) -> Self {
        self.output_values
            .push(bytemuck::cast_slice(output_values).to_vec());

        self
    }

    fn failures(mut self, failures: Backends) -> Self {
        self.failures = failures;

        self
    }
}

const MAX_BUFFER_SIZE: u64 = 128;

/// Runs the given shader tests with the given storage_type for the input_buffer.
async fn shader_input_output_test(
    ctx: TestingContext,
    storage_type: InputStorageType,
    tests: Vec<ShaderTest>,
) {
    let source = String::from(include_str!("shader_test.wgsl"));

    for test in &tests {
        assert!(test.buffer_sizes_valid(), "invalid buffers: {}", test.name);
    }

    let input_buffer = ctx.device.create_buffer(&BufferDescriptor {
        label: Some("input buffer"),
        size: MAX_BUFFER_SIZE,
        usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM | BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let output_buffer = ctx.device.create_buffer(&BufferDescriptor {
        label: Some("output buffer"),
        size: MAX_BUFFER_SIZE,
        usage: BufferUsages::COPY_DST | BufferUsages::COPY_SRC | BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let mut fail = false;
    for test in tests {
        let explicit_buffers = test.module_buffers.is_some();
        let test_name = test.name;

        // -- Building shader + pipeline --

        // This isn't terribly efficient but the string is short and it's a test.
        // The body and input members are the longest part, so do them last.
        let processed = if let Some(module_source) = test.module_source {
            module_source
        } else {
            let processed = source
                .replace("{{header}}", &test.header)
                .replace("{{storage_type}}", storage_type.as_str())
                .replace("{{input_type}}", &test.input_type)
                .replace("{{output_type}}", &test.output_type)
                .replace("{{input_members}}", &test.custom_struct_members)
                .replace("{{body}}", &test.body);

            // Add the bindings for all inputs besides immediates.
            if matches!(storage_type, InputStorageType::Immediate) {
                processed.replace("{{input_bindings}}", "")
            } else {
                processed.replace("{{input_bindings}}", "@group(0) @binding(0)")
            }
        };

        let (buffers, output_index, workgroups) = if let Some(module) = test.module_buffers {
            let output_index = module
                .buffers
                .iter()
                .position(|buffer| buffer.binding == module.output_binding)
                .unwrap();
            let buffers: Vec<_> = module
                .buffers
                .into_iter()
                .map(|buffer| {
                    let mut usage = BufferUsages::COPY_DST
                        | match buffer.ty {
                            wgpu::BufferBindingType::Uniform => BufferUsages::UNIFORM,
                            wgpu::BufferBindingType::Storage { .. } => BufferUsages::STORAGE,
                        };
                    if buffer.binding == module.output_binding {
                        usage |= BufferUsages::COPY_SRC;
                    }
                    let resource = ctx.device.create_buffer_init(&BufferInitDescriptor {
                        label: Some(&test_name),
                        contents: &buffer.contents,
                        usage,
                    });
                    (buffer.binding, buffer.ty, resource)
                })
                .collect();
            (buffers, output_index, module.workgroups)
        } else {
            let input_ty = match storage_type {
                InputStorageType::Uniform => wgpu::BufferBindingType::Uniform,
                InputStorageType::Storage | InputStorageType::Immediate => {
                    wgpu::BufferBindingType::Storage { read_only: true }
                }
            };
            (
                vec![
                    (0, input_ty, input_buffer.clone()),
                    (
                        1,
                        wgpu::BufferBindingType::Storage { read_only: false },
                        output_buffer.clone(),
                    ),
                ],
                1,
                [1; 3],
            )
        };
        let output = &buffers[output_index].2;
        let layout_entries: Vec<_> = buffers
            .iter()
            .map(|&(binding, ty, _)| BindGroupLayoutEntry {
                binding,
                visibility: ShaderStages::COMPUTE,
                ty: BindingType::Buffer {
                    ty,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            })
            .collect();
        let bgl = ctx
            .device
            .create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &layout_entries,
            });
        let entries: Vec<_> = buffers
            .iter()
            .map(|(binding, _, buffer)| BindGroupEntry {
                binding: *binding,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        let bg = ctx.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &bgl,
            entries: &entries,
        });
        let pll = ctx
            .device
            .create_pipeline_layout(&PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[Some(&bgl)],
                immediate_size: match storage_type {
                    InputStorageType::Immediate => MAX_BUFFER_SIZE as u32,
                    _ => 0,
                },
            });
        let mapping_buffer = ctx.device.create_buffer(&BufferDescriptor {
            label: Some("mapping buffer"),
            size: output.size(),
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let sm = ctx.device.create_shader_module(ShaderModuleDescriptor {
            label: Some(&format!("shader {test_name}")),
            source: ShaderSource::Wgsl(Cow::Borrowed(&processed)),
        });

        let pipeline = ctx
            .device
            .create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some(&format!("pipeline {test_name}")),
                layout: Some(&pll),
                module: &sm,
                entry_point: Some(&test.entry_point),
                compilation_options: Default::default(),
                cache: None,
            });

        // -- Initializing data --

        if !explicit_buffers {
            let mut output_pre_init_data =
                vec![test.output_initialization; MAX_BUFFER_SIZE as usize / 4];
            output_pre_init_data[..test.output_initial_values.len()]
                .copy_from_slice(&test.output_initial_values);
            ctx.queue.write_buffer(
                &output_buffer,
                0,
                bytemuck::cast_slice(&output_pre_init_data),
            );

            match storage_type {
                InputStorageType::Uniform | InputStorageType::Storage => {
                    ctx.queue.write_buffer(
                        &input_buffer,
                        0,
                        bytemuck::cast_slice(&test.input_values),
                    );
                }
                _ => {
                    // Init happens in the compute pass
                }
            }
        }

        // -- Run test --

        let mut encoder = ctx
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        let mut cpass = encoder.begin_compute_pass(&ComputePassDescriptor {
            label: Some(&format!("cpass {test_name}")),
            timestamp_writes: None,
        });
        cpass.set_pipeline(&pipeline);
        cpass.set_bind_group(0, &bg, &[]);

        if let InputStorageType::Immediate = storage_type {
            cpass.set_immediates(0, bytemuck::cast_slice(&test.input_values))
        }

        cpass.dispatch_workgroups(workgroups[0], workgroups[1], workgroups[2]);
        drop(cpass);

        // -- Pulldown data --

        encoder.copy_buffer_to_buffer(output, 0, &mapping_buffer, 0, output.size());

        ctx.queue.submit(Some(encoder.finish()));

        mapping_buffer.slice(..).map_async(MapMode::Read, |_| ());
        ctx.async_poll(PollType::wait_indefinitely()).await.unwrap();

        let mapped = mapping_buffer.slice(..).get_mapped_range().unwrap();

        let typed: &[u32] = bytemuck::cast_slice(&mapped);

        // -- Check results --

        let failure = !(test.output_comparison_fn)(&test_name, typed, &test.output_values);
        // We don't immediately panic to let all tests execute
        if failure
            != test
                .failures
                .contains(ctx.adapter.get_info().backend.into())
        {
            fail |= true;
            if !failure {
                eprintln!("Unexpected test success. Test {test_name}");
            }
        }

        drop(mapped);
        mapping_buffer.unmap();
    }
    assert!(!fail);
}
