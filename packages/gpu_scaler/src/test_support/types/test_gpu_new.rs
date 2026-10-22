use super::TestGpu;

impl TestGpu {
    pub fn new() -> Option<TestGpu> {
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }));
        let Ok(adapter) = adapter else {
            println!("no gpu adapter");
            return None;
        };
        let info = adapter.get_info();
        println!("gpu adapter: {} on {:?}", info.name, info.backend);
        let device = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor::default()));
        let (device, queue) = match device {
            Ok(device_and_queue) => device_and_queue,
            Err(error) => {
                println!("no gpu device on {}: {error}", info.name);
                return None;
            }
        };
        Some(TestGpu { device, queue })
    }
}
