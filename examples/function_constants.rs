use minmetal::*;

const SHADER: &str = r#"
#include <metal_stdlib>
using namespace metal;

constant uint FACTOR [[function_constant(0)]];

kernel void constant_kernel(device uint* values [[buffer(0)]],
                            uint index [[thread_position_in_grid]]) {
    values[index] = index * FACTOR;
}
"#;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let device = Device::system_default().ok_or("no Metal device is available")?;
    let library = device.new_library_with_source(SHADER)?;

    let constants = FunctionConstantValues::new();
    constants.set_u32(0, 5);

    let function = library.function_with_constants("constant_kernel", &constants)?;
    let _pipeline = device.new_compute_pipeline_state_with_function(&function)?;

    println!("function constants smoke test passed");
    Ok(())
}
