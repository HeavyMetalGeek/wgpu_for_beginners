pub struct Pipeline {
    shader_filename: String,
    vertex_entry: String,
    fragment_entry: String,
    pixel_format: wgpu::TextureFormat,
}

impl Default for Pipeline {
    fn default() -> Self {
        Self {
            shader_filename: String::default(),
            vertex_entry: String::default(),
            fragment_entry: String::default(),
            pixel_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }
}

impl Pipeline {
    pub fn new(shader_filename: &str, vertex_entry: &str, fragment_entry: &str) -> Self {
        Self {
            shader_filename: shader_filename.into(),
            vertex_entry: vertex_entry.into(),
            fragment_entry: fragment_entry.into(),
            pixel_format: wgpu::TextureFormat::Bgra8UnormSrgb,
        }
    }

    pub fn set_pixel_format(mut self, pixel_format: wgpu::TextureFormat) -> Self {
        self.pixel_format = pixel_format;
        self
    }

    pub fn build_pipeline(&self, device: &wgpu::Device) -> wgpu::RenderPipeline {
        let filepath = std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR"))
            .join("assets")
            .join(self.shader_filename.as_str());
        if !filepath.exists() {
            panic!("Shader path does not exist: {:?}", filepath);
        }
        let shader_source = std::fs::read_to_string(filepath).expect("Unable to read shader file.");

        let shader_module_descriptor = wgpu::ShaderModuleDescriptor {
            label: Some("Shader Module"),
            source: wgpu::ShaderSource::Wgsl(shader_source.into()),
        };
        let shader_module = device.create_shader_module(shader_module_descriptor);

        let pipeline_layout_descriptor = wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };
        let pipeline_layout = device.create_pipeline_layout(&pipeline_layout_descriptor);

        let render_targets = [Some(wgpu::ColorTargetState {
            format: self.pixel_format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: &self.vertex_entry,
                buffers: &[],
                compilation_options: Default::default(),
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },

            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: &self.fragment_entry,
                targets: &render_targets,
                compilation_options: Default::default(),
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        };

        device.create_render_pipeline(&render_pipeline_descriptor)
    }
}
