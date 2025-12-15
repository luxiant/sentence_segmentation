#[cfg(feature = "thai")]
use burn_import::onnx::ModelGen;

fn main() {
    #[cfg(feature = "thai")]
    ModelGen::new()
        .input("model/thai_segmenter.onnx") 
        .out_dir("model/")
        .run_from_script();
}