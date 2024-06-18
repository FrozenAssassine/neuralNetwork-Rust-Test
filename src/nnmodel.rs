use crate::{layer::Layer, neuralnetwork::NeuralNetwork};


pub struct NNModel{
    neural_network: NeuralNetwork
}

impl NNModel {
    fn new(input:Layer, hidden: Vec<Layer>, output: Layer) -> NNModel{

        NNModel{
            neural_network: NeuralNetwork::new(input, hidden, output)
        }
    }

    fn 
}