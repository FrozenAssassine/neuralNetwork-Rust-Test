use rand::Rng;
use std::rc::Rc;

pub struct Layer{
    pub size: usize,
    pub biases: Vec<f32>, 
    pub weights: Vec<f32>, 
    pub errors: Vec<f32>, 
    pub values: Vec<f32>,
    pub prev_layer: Option<Rc<Layer>>
}

impl Layer{
    fn new(size: usize) -> Layer{
        Layer { 
            size: size,
            biases: Vec::with_capacity(size), 
            weights: Vec::new(), 
            errors: Vec::with_capacity(size), 
            values: Vec::with_capacity(size),
        } 
    }


    pub fn init(mut self, previous_layer: Option<Layer>){
        self.fill_random();
        
        if previous_layer.is_none() {
            return;
        }

        let size = self.size * previous_layer.unwrap().size;
        self.weights = Vec::with_capacity(size);
        self.prev_layer  = previous_layer;  
    }

    fn fill_random(&mut self){
        let mut rng = rand::thread_rng();
        
        for bias in &mut self.biases {
            *bias = rng.gen_range(-1.0..=1.0);
        }

        for weight in &mut self.weights {
            *weight = rng.gen_range(-1.0..=1.0);
        }
    }
}