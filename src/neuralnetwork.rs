
use crate::layer::Layer;

pub struct NeuralNetwork{
    input: Layer,
    hidden: Vec<Layer>,
    output: Layer
}

impl NeuralNetwork{
    pub fn new(input: Layer, hidden: Vec<Layer>, output: Layer) -> NeuralNetwork{
        NeuralNetwork{
            hidden: hidden,
            output: output,
            input: input
        }
    }

    pub fn init(&mut self){
        self.input.init(None);
        
        for (index, hid) in self.hidden.iter().enumerate() {
            if index == 0 {
                hid.init(Some(self.input));
                continue;
            }

            hid.init(Some(self.hidden[index - 1]));
        }
        self.output.init(Some(self.hidden[self.hidden.len() -1]));
    }

    pub fn Train(inputs: Vec<f32>, desired: Vec<f32>, learning_rate: f32){

    }

    pub fn FeedForward(&mut self, data: Vec<f32>){
        for i in 0..=data.len(){
            self.input.values[i] = data[i];
        }

        for hidden in &self.hidden {
            for i in 0..hidden.size{
                let sum: f32 = 0.0;
                for j in 0..hidden.prev_layer.
            }
        }
    }
}