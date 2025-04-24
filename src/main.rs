mod perceptron;
mod mnist;

use ::mnist::Mnist;
use crate::mnist::mnist_manager::MnistManager;

fn main() {
    let mnist_manager = MnistManager::new();
    mnist_manager.load_mnist_data();
}
