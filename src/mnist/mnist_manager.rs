use mnist::{MnistBuilder, Mnist};

pub struct MnistManager {

}

impl MnistManager {
    pub fn new() -> Self {
        MnistManager {}
    }

    pub fn load_mnist_data(&self) -> Mnist {
        let mnist = MnistBuilder::new()
            .download_and_extract()
            .finalize();
        mnist
    }
}