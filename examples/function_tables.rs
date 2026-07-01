use minmetal::*;

fn main() {
    println!("Running function_tables example...");
    let Some(device) = Device::system_default() else {
        println!("No Metal device available.");
        return;
    };

    let library = device.new_library_with_source(r#"
        #include <metal_stdlib>
        using namespace metal;
        kernel void compute_main() {}
    "#);

    if let Ok(lib) = library {
        if let Ok(func) = lib.function("compute_main") {
            let desc = ComputePipelineDescriptor::new();
            desc.set_compute_function(&func);

            if let Ok(state) = device.new_compute_pipeline_state(&desc) {
                println!("Compute pipeline state created successfully.");

                // Create a visible function table descriptor
                let table_desc = VisibleFunctionTableDescriptor::new();
                table_desc.set_function_count(4);

                if let Ok(table) = state.new_visible_function_table(&table_desc) {
                    println!("Successfully created VisibleFunctionTable from ComputePipelineState!");
                    if let Ok(_resource_id) = table.gpu_resource_id() {
                        println!("VisibleFunctionTable GPU resource ID is present.");
                    }
                }

                let isect_desc = IntersectionFunctionTableDescriptor::new();
                isect_desc.set_function_count(4);
                if let Ok(_table) = state.new_intersection_function_table(&isect_desc) {
                    println!("Successfully created IntersectionFunctionTable from ComputePipelineState!");
                }
            }
        }
    }
}
