use std::path::Path;
use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

fn main() {
    let shader_paths = vec!["shaders/shader.vert", "shaders/shader.frag"];

    let mut compiler = shaderc::Compiler::new().expect("Able to create shader compiler");

    // This can't be parallelized. The [shaderc::Compiler] is not
    // thread safe. Also, it creates a lot of resources. You could
    // spawn multiple processes to handle this, but it would probably
    // be better just to only compile shaders that have been changed
    // recently.
    for shader_path in shader_paths {
        let shader = ShaderData::load(shader_path);
        let compiled = compiler
            .compile_into_spirv(
                &shader.src,
                shader.kind,
                &shader.src_path.to_str().unwrap(),
                "main",
                None,
            )
            .unwrap();
        write(shader.spv_path, compiled.as_binary_u8()).unwrap();
        println!("cargo:rerun-if-changed={}", shader_path);
    }
}

struct ShaderData {
    src: String,
    src_path: PathBuf,
    spv_path: PathBuf,
    kind: shaderc::ShaderKind,
}

impl ShaderData {
    pub fn load(src_path: impl AsRef<Path>) -> Self {
        let src_path: &Path = src_path.as_ref();
        let extension = src_path
            .extension()
            .expect("File has extension")
            .to_str()
            .expect("Extension can be converted to &str");
        let kind = match extension {
            "vert" => shaderc::ShaderKind::Vertex,
            "frag" => shaderc::ShaderKind::Fragment,
            "comp" => shaderc::ShaderKind::Compute,
            _ => panic!("Unsupported shader: {}", src_path.display()),
        };

        let src = read_to_string(src_path).unwrap();
        let spv_path = src_path.with_extension(format!("{}.spv", extension));

        Self {
            src,
            src_path: src_path.to_path_buf(),
            spv_path,
            kind,
        }
    }
}
