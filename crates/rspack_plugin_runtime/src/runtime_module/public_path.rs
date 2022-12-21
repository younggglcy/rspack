use rspack_core::{
  rspack_sources::{BoxSource, RawSource, SourceExt},
  Compilation, PublicPath, RuntimeModule,
};

#[derive(Debug, Default)]
pub struct PublicPathRuntimeModule {}

impl RuntimeModule for PublicPathRuntimeModule {
  fn identifier(&self) -> String {
    "webpack/runtime/public_path".to_string()
  }

  fn generate(&self, compilation: &Compilation) -> BoxSource {
    match &compilation.options.output.public_path {
      PublicPath::String(str) => RawSource::from(
        include_str!("runtime/public_path.js").replace("__PUBLIC_PATH_PLACEHOLDER__", str),
      )
      .boxed(),
      PublicPath::Auto => RawSource::from(include_str!("runtime/public_path_auto.js")).boxed(),
    }
  }
}
