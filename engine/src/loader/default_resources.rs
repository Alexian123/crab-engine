use super::EmbeddedShader;

pub const DEFAULT_UI_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/ui.vert"),
    fragment_path: concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/ui.frag"),
};

pub const DEFAULT_3D_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/static.vert"),
    fragment_path: concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/static.frag"),
};

pub const DEFAULT_SKYBOX_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/skybox.vert"),
    fragment_path: concat!(env!("CARGO_MANIFEST_DIR"), "/shaders/skybox.frag"),
};

pub const DEFAULT_PRESENT_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/present.vert"
    ),
    fragment_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/present.frag"
    ),
};

pub const DEFAULT_CONTRAST_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/contrast/contrast.vert"
    ),
    fragment_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/contrast/contrast.frag"
    ),
};

pub const DEFAULT_HBLUR_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/blur/hblur.vert"
    ),
    fragment_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/blur/blur.frag"
    ),
};

pub const DEFAULT_VBLUR_SHADER: EmbeddedShader = EmbeddedShader {
    vertex_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/blur/vblur.vert"
    ),
    fragment_path: concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/shaders/postprocessing/blur/blur.frag"
    ),
};
