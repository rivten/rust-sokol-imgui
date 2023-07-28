use glfw::Context;

macro_rules! include_cstr {
    ( $path:literal $(,)? ) => {{
        // Use a constant to force the verification to run at compile time.
        const VALUE: *const ::core::ffi::c_char =
            $crate::to_cstr(concat!(include_str!($path), "\0"));
        VALUE
    }};
}

const fn to_cstr(s: &str) -> *const core::ffi::c_char {
    let bytes = s.as_bytes();
    let mut i = 0;
    while i < (bytes.len() - 1) {
        assert!(bytes[i] != b'\0', "interior byte cannot be NUL");
        i += 1;
    }
    assert!(bytes[bytes.len() - 1] == b'\0', "last byte must be NUL");
    // SAFETY: We verified there are no interior NULs and that the string ends with NUL.
    bytes.as_ptr() as *const core::ffi::c_char
}

const IMGUI_BUFFER_SIZE: usize = 2 * 2 * 2048;

fn glfw_mouse_button_to_imgui_button(glfw_mouse_button: glfw::MouseButton) -> imgui::MouseButton {
    match glfw_mouse_button {
        glfw::MouseButton::Button1 => imgui::MouseButton::Left,
        glfw::MouseButton::Button2 => imgui::MouseButton::Right,
        glfw::MouseButton::Button3 => imgui::MouseButton::Middle,
        glfw::MouseButton::Button4 => imgui::MouseButton::Extra1,
        glfw::MouseButton::Button5 => imgui::MouseButton::Extra2,
        glfw::MouseButton::Button6 => panic!(),
        glfw::MouseButton::Button7 => panic!(),
        glfw::MouseButton::Button8 => panic!(),
    }
}

fn glfw_key_to_imgui_key(glfw_keycode: glfw::Key) -> imgui::Key {
    match glfw_keycode {
        glfw::Key::Space => imgui::Key::Space,
        glfw::Key::Apostrophe => imgui::Key::Apostrophe,
        glfw::Key::Comma => imgui::Key::Comma,
        glfw::Key::Minus => imgui::Key::Minus,
        glfw::Key::Period => imgui::Key::Period,
        glfw::Key::Slash => imgui::Key::Slash,
        glfw::Key::Num0 => imgui::Key::Keypad0,
        glfw::Key::Num1 => imgui::Key::Keypad1,
        glfw::Key::Num2 => imgui::Key::Keypad2,
        glfw::Key::Num3 => imgui::Key::Keypad3,
        glfw::Key::Num4 => imgui::Key::Keypad4,
        glfw::Key::Num5 => imgui::Key::Keypad5,
        glfw::Key::Num6 => imgui::Key::Keypad6,
        glfw::Key::Num7 => imgui::Key::Keypad7,
        glfw::Key::Num8 => imgui::Key::Keypad8,
        glfw::Key::Num9 => imgui::Key::Keypad9,
        glfw::Key::Semicolon => imgui::Key::Semicolon,
        glfw::Key::Equal => imgui::Key::Equal,
        glfw::Key::A => imgui::Key::A,
        glfw::Key::B => imgui::Key::B,
        glfw::Key::C => imgui::Key::C,
        glfw::Key::D => imgui::Key::D,
        glfw::Key::E => imgui::Key::E,
        glfw::Key::F => imgui::Key::F,
        glfw::Key::G => imgui::Key::G,
        glfw::Key::H => imgui::Key::H,
        glfw::Key::I => imgui::Key::I,
        glfw::Key::J => imgui::Key::J,
        glfw::Key::K => imgui::Key::K,
        glfw::Key::L => imgui::Key::L,
        glfw::Key::M => imgui::Key::M,
        glfw::Key::N => imgui::Key::N,
        glfw::Key::O => imgui::Key::O,
        glfw::Key::P => imgui::Key::P,
        glfw::Key::Q => imgui::Key::Q,
        glfw::Key::R => imgui::Key::R,
        glfw::Key::S => imgui::Key::S,
        glfw::Key::T => imgui::Key::T,
        glfw::Key::U => imgui::Key::U,
        glfw::Key::V => imgui::Key::V,
        glfw::Key::W => imgui::Key::W,
        glfw::Key::X => imgui::Key::X,
        glfw::Key::Y => imgui::Key::Y,
        glfw::Key::Z => imgui::Key::Z,
        glfw::Key::LeftBracket => imgui::Key::LeftBracket,
        glfw::Key::Backslash => imgui::Key::Backslash,
        glfw::Key::RightBracket => imgui::Key::RightBracket,
        glfw::Key::GraveAccent => imgui::Key::GraveAccent,
        glfw::Key::World1 => panic!(),
        glfw::Key::World2 => panic!(),
        glfw::Key::Escape => imgui::Key::Escape,
        glfw::Key::Enter => imgui::Key::Enter,
        glfw::Key::Tab => imgui::Key::Tab,
        glfw::Key::Backspace => imgui::Key::Backspace,
        glfw::Key::Insert => imgui::Key::Insert,
        glfw::Key::Delete => imgui::Key::Delete,
        glfw::Key::Right => imgui::Key::RightArrow,
        glfw::Key::Left => imgui::Key::LeftArrow,
        glfw::Key::Down => imgui::Key::DownArrow,
        glfw::Key::Up => imgui::Key::UpArrow,
        glfw::Key::PageUp => imgui::Key::PageUp,
        glfw::Key::PageDown => imgui::Key::PageDown,
        glfw::Key::Home => imgui::Key::Home,
        glfw::Key::End => imgui::Key::End,
        glfw::Key::CapsLock => imgui::Key::CapsLock,
        glfw::Key::ScrollLock => imgui::Key::ScrollLock,
        glfw::Key::NumLock => imgui::Key::NumLock,
        glfw::Key::PrintScreen => imgui::Key::PrintScreen,
        glfw::Key::Pause => imgui::Key::Pause,
        glfw::Key::F1 => imgui::Key::F1,
        glfw::Key::F2 => imgui::Key::F2,
        glfw::Key::F3 => imgui::Key::F3,
        glfw::Key::F4 => imgui::Key::F4,
        glfw::Key::F5 => imgui::Key::F5,
        glfw::Key::F6 => imgui::Key::F6,
        glfw::Key::F7 => imgui::Key::F7,
        glfw::Key::F8 => imgui::Key::F8,
        glfw::Key::F9 => imgui::Key::F9,
        glfw::Key::F10 => imgui::Key::F10,
        glfw::Key::F11 => imgui::Key::F11,
        glfw::Key::F12 => imgui::Key::F12,
        glfw::Key::F13 => panic!(),
        glfw::Key::F14 => panic!(),
        glfw::Key::F15 => panic!(),
        glfw::Key::F16 => panic!(),
        glfw::Key::F17 => panic!(),
        glfw::Key::F18 => panic!(),
        glfw::Key::F19 => panic!(),
        glfw::Key::F20 => panic!(),
        glfw::Key::F21 => panic!(),
        glfw::Key::F22 => panic!(),
        glfw::Key::F23 => panic!(),
        glfw::Key::F24 => panic!(),
        glfw::Key::F25 => panic!(),
        glfw::Key::Kp0 => imgui::Key::Keypad0,
        glfw::Key::Kp1 => imgui::Key::Keypad1,
        glfw::Key::Kp2 => imgui::Key::Keypad2,
        glfw::Key::Kp3 => imgui::Key::Keypad3,
        glfw::Key::Kp4 => imgui::Key::Keypad4,
        glfw::Key::Kp5 => imgui::Key::Keypad5,
        glfw::Key::Kp6 => imgui::Key::Keypad6,
        glfw::Key::Kp7 => imgui::Key::Keypad7,
        glfw::Key::Kp8 => imgui::Key::Keypad8,
        glfw::Key::Kp9 => imgui::Key::Keypad9,
        glfw::Key::KpDecimal => imgui::Key::KeypadDecimal,
        glfw::Key::KpDivide => imgui::Key::KeypadDivide,
        glfw::Key::KpMultiply => imgui::Key::KeypadMultiply,
        glfw::Key::KpSubtract => imgui::Key::KeypadSubtract,
        glfw::Key::KpAdd => imgui::Key::KeypadAdd,
        glfw::Key::KpEnter => imgui::Key::KeypadEnter,
        glfw::Key::KpEqual => imgui::Key::KeypadEqual,
        glfw::Key::LeftShift => imgui::Key::LeftShift,
        glfw::Key::LeftControl => imgui::Key::LeftCtrl,
        glfw::Key::LeftAlt => imgui::Key::LeftAlt,
        glfw::Key::LeftSuper => imgui::Key::LeftSuper,
        glfw::Key::RightShift => imgui::Key::RightShift,
        glfw::Key::RightControl => imgui::Key::RightCtrl,
        glfw::Key::RightAlt => imgui::Key::RightAlt,
        glfw::Key::RightSuper => imgui::Key::RightSuper,
        glfw::Key::Menu => imgui::Key::Menu,
        glfw::Key::Unknown => panic!(),
    }
}

fn render_imgui(
    cur_fb_width: i32,
    cur_fb_height: i32,
    imgui_pip: sokol::gfx::Pipeline,
    imgui_vtx_buf: sokol::gfx::Buffer,
    imgui_idx_buf: sokol::gfx::Buffer,
    imgui: &mut imgui::Context,
) {
    let mut vertices: Vec<imgui::DrawVert> = Vec::new();
    let mut indices: Vec<imgui::DrawIdx> = Vec::new();
    let font_image = sokol::gfx::Image {
        id: imgui.fonts().tex_id.id() as u32,
    };
    let display_size = imgui.io().display_size;
    let draw_lists: Vec<_> = imgui.render().draw_lists().collect();
    for draw_list in &draw_lists {
        for v in draw_list.vtx_buffer() {
            vertices.push(v.clone());
        }
        for i in draw_list.idx_buffer() {
            indices.push(i.clone());
        }
    }

    assert!(vertices.len() < IMGUI_BUFFER_SIZE);
    assert!(indices.len() < IMGUI_BUFFER_SIZE);

    sokol::gfx::update_buffer(imgui_vtx_buf, &sokol::gfx::slice_as_range(&vertices));
    sokol::gfx::update_buffer(imgui_idx_buf, &sokol::gfx::slice_as_range(&indices));

    sokol::gfx::apply_viewport(0, 0, cur_fb_width, cur_fb_height, true);
    sokol::gfx::apply_scissor_rect(0, 0, cur_fb_width, cur_fb_height, true);
    sokol::gfx::apply_pipeline(imgui_pip);

    sokol::gfx::apply_uniforms(
        sokol::gfx::ShaderStage::Vs,
        0,
        &sokol::gfx::slice_as_range(&display_size),
    );

    let mut bind = sokol::gfx::Bindings::new();
    bind.vertex_buffers[0] = imgui_vtx_buf;
    bind.index_buffer = imgui_idx_buf;
    bind.fs_images[0] = font_image;

    let mut vb_offset = 0;
    let mut ib_offset = 0;

    for draw_list in draw_lists {
        bind.vertex_buffer_offsets[0] = vb_offset;
        bind.index_buffer_offset = ib_offset;
        sokol::gfx::apply_bindings(&bind);
        for draw_cmd in draw_list.commands() {
            match draw_cmd {
                imgui::DrawCmd::Elements { count, cmd_params } => {
                    assert!(cmd_params.vtx_offset == 0);
                    let scissor_x = cmd_params.clip_rect[0] as i32;
                    let scissor_y = cmd_params.clip_rect[1] as i32;
                    let scissor_w = (cmd_params.clip_rect[2] - cmd_params.clip_rect[0]) as i32;
                    let scissor_h = (cmd_params.clip_rect[3] - cmd_params.clip_rect[1]) as i32;
                    sokol::gfx::apply_scissor_rect(
                        scissor_x, scissor_y, scissor_w, scissor_h, true,
                    );
                    sokol::gfx::draw(cmd_params.idx_offset, count, 1);
                }
                _ => panic!(),
            }
        }
        vb_offset += (std::mem::size_of::<imgui::DrawVert>() * draw_list.vtx_buffer().len()) as i32;
        ib_offset += (std::mem::size_of::<imgui::DrawIdx>() * draw_list.idx_buffer().len()) as i32;
    }
}

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));

    let (mut window, events) = glfw
        .create_window(300, 300, "gyoma", glfw::WindowMode::Windowed)
        .unwrap();

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);

    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    sokol::gfx::setup(&sokol::gfx::Desc {
        logger: sokol::gfx::Logger {
            func: Some(sokol::log::slog_func),
            ..Default::default()
        },
        ..Default::default()
    });

    #[rustfmt::skip]
    let vertices = [
        0.0f32, 0.5, 0.5,    1.0, 0.0, 0.0, 1.0,
        0.5, -0.5, 0.5,   0.0, 1.0, 0.0, 1.0,
        -0.5, -0.5, 0.5,  0.0, 0.0, 1.0, 1.0,
    ];

    let vbuf = sokol::gfx::make_buffer(&sokol::gfx::BufferDesc {
        data: sokol::gfx::slice_as_range(&vertices),
        ..Default::default()
    });

    let shader = sokol::gfx::make_shader(&sokol::gfx::ShaderDesc {
        vs: sokol::gfx::ShaderStageDesc {
            source: include_cstr!("shaders/vs.glsl"),
            ..Default::default()
        },
        fs: sokol::gfx::ShaderStageDesc {
            source: include_cstr!("shaders/fs.glsl"),
            ..Default::default()
        },
        ..Default::default()
    });

    let pip = sokol::gfx::make_pipeline(&sokol::gfx::PipelineDesc {
        shader,
        layout: {
            let mut layout = sokol::gfx::LayoutDesc::new();
            layout.attrs[0].format = sokol::gfx::VertexFormat::Float3;
            layout.attrs[1].format = sokol::gfx::VertexFormat::Float4;
            layout
        },
        ..Default::default()
    });

    let bind = sokol::gfx::Bindings {
        vertex_buffers: {
            let mut v = [sokol::gfx::Buffer::new(); 8];
            v[0] = vbuf;

            v
        },
        ..Default::default()
    };

    let mut imgui = imgui::Context::create();
    imgui
        .fonts()
        .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

    let imgui_font_atlas_texture = imgui.fonts().build_rgba32_texture();
    let imgui_font_image = sokol::gfx::make_image(&sokol::gfx::ImageDesc {
        width: imgui_font_atlas_texture.width as i32,
        height: imgui_font_atlas_texture.height as i32,
        pixel_format: sokol::gfx::PixelFormat::Rgba8,
        wrap_u: sokol::gfx::Wrap::ClampToEdge,
        wrap_v: sokol::gfx::Wrap::ClampToEdge,
        min_filter: sokol::gfx::Filter::Linear,
        mag_filter: sokol::gfx::Filter::Linear,
        data: {
            let mut d = sokol::gfx::ImageData::new();
            d.subimage[0][0] = sokol::gfx::slice_as_range(imgui_font_atlas_texture.data);
            d
        },
        ..Default::default()
    });
    imgui.fonts().tex_id = imgui::TextureId::new(imgui_font_image.id as usize);

    let imgui_shader = sokol::gfx::make_shader(&sokol::gfx::ShaderDesc {
        vs: sokol::gfx::ShaderStageDesc {
            source: include_cstr!("shaders/imgui_vs.glsl"),
            uniform_blocks: {
                let mut uniforms = [sokol::gfx::ShaderUniformBlockDesc::new(); 4];
                uniforms[0].size = 2 * std::mem::size_of::<f32>();
                uniforms[0].uniforms[0]._type = sokol::gfx::UniformType::Float2;
                uniforms[0].uniforms[0].array_count = 1;
                uniforms
            },
            ..Default::default()
        },
        fs: sokol::gfx::ShaderStageDesc {
            source: include_cstr!("shaders/imgui_fs.glsl"),
            images: {
                let mut imgs = [sokol::gfx::ShaderImageDesc::new(); 12];
                imgs[0].image_type = sokol::gfx::ImageType::Dim2;
                imgs[0].sampler_type = sokol::gfx::SamplerType::Float;
                imgs
            },
            ..Default::default()
        },
        ..Default::default()
    });

    let imgui_pip = sokol::gfx::make_pipeline(&sokol::gfx::PipelineDesc {
        shader: imgui_shader,
        index_type: sokol::gfx::IndexType::Uint16,
        colors: {
            let mut colors = [sokol::gfx::ColorState::new(); 4];
            colors[0].write_mask = sokol::gfx::ColorMask::Rgba;
            colors[0].blend.enabled = true;
            colors[0].blend.src_factor_rgb = sokol::gfx::BlendFactor::SrcAlpha;
            colors[0].blend.dst_factor_rgb = sokol::gfx::BlendFactor::OneMinusSrcAlpha;
            colors
        },
        layout: {
            let mut layout = sokol::gfx::LayoutDesc::new();
            layout.buffers[0].stride = std::mem::size_of::<imgui::DrawVert>() as i32;
            layout.attrs[0].format = sokol::gfx::VertexFormat::Float2;
            layout.attrs[0].offset = 0;
            layout.attrs[1].format = sokol::gfx::VertexFormat::Float2;
            layout.attrs[1].offset = 2 * std::mem::size_of::<f32>() as i32;
            layout.attrs[2].format = sokol::gfx::VertexFormat::Ubyte4n;
            layout.attrs[2].offset = 4 * std::mem::size_of::<f32>() as i32;
            layout
        },
        ..Default::default()
    });

    let imgui_vtx_buf = sokol::gfx::make_buffer(&sokol::gfx::BufferDesc {
        size: std::mem::size_of::<imgui::DrawVert>() * IMGUI_BUFFER_SIZE,
        usage: sokol::gfx::Usage::Stream,
        _type: sokol::gfx::BufferType::Vertexbuffer,
        ..Default::default()
    });

    let imgui_idx_buf = sokol::gfx::make_buffer(&sokol::gfx::BufferDesc {
        size: std::mem::size_of::<imgui::DrawIdx>() * IMGUI_BUFFER_SIZE,
        usage: sokol::gfx::Usage::Stream,
        _type: sokol::gfx::BufferType::Indexbuffer,
        ..Default::default()
    });

    let mut last_time = glfw.get_time();

    while !window.should_close() {
        let (cur_fb_width, cur_fb_height) = window.get_framebuffer_size();
        let (cur_win_width, cur_win_height) = window.get_size();

        // new frame imgui
        let mut io = imgui.io_mut();
        io.display_size = [cur_win_width as f32, cur_win_height as f32];
        io.display_framebuffer_scale = [
            cur_fb_width as f32 / cur_win_width as f32,
            cur_fb_height as f32 / cur_win_height as f32,
        ];

        let current_time = glfw.get_time();
        io.delta_time = current_time as f32 - last_time as f32;
        last_time = glfw.get_time();

        let (mouse_x, mouse_y) = window.get_cursor_pos();
        io.add_mouse_pos_event([mouse_x as f32, mouse_y as f32]);

        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true)
                }
                glfw::WindowEvent::Key(keycode, _, action, _) => {
                    io.add_key_event(
                        glfw_key_to_imgui_key(keycode),
                        action == glfw::Action::Press,
                    );
                }
                glfw::WindowEvent::MouseButton(button, action, _) => io.add_mouse_button_event(
                    glfw_mouse_button_to_imgui_button(button),
                    action == glfw::Action::Press,
                ),
                _ => {}
            }
        }
        let imgui_ui = imgui.new_frame();
        let mut show = true;
        imgui_ui.show_demo_window(&mut show);

        let mut pass_action = sokol::gfx::PassAction::new();
        pass_action.colors[0] = sokol::gfx::ColorAttachmentAction {
            load_action: sokol::gfx::LoadAction::Clear,
            clear_value: sokol::gfx::Color {
                r: 0.25,
                g: 0.5,
                b: 0.75,
                a: 1.0,
            },
            ..Default::default()
        };
        sokol::gfx::begin_default_pass(&pass_action, cur_fb_width, cur_fb_height);
        sokol::gfx::apply_pipeline(pip);
        sokol::gfx::apply_bindings(&bind);
        sokol::gfx::draw(0, 3, 1);

        // render imgui
        render_imgui(
            cur_fb_width,
            cur_fb_height,
            imgui_pip,
            imgui_vtx_buf,
            imgui_idx_buf,
            &mut imgui,
        );
        sokol::gfx::end_pass();
        sokol::gfx::commit();

        window.swap_buffers();
    }

    sokol::gfx::shutdown();
}
