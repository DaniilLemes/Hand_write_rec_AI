pub struct Perceptron {
    alfa: f64,
    threshold: f64,
    weights: Vec<f64>,
}

impl Perceptron {

    pub fn new(
        weights_vector_length: i64,
        alfa: f64,
        threshold: f64,
    ) -> Self {
        Self {
            alfa,
            threshold,
            weights: vec![0.0; weights_vector_length as usize], //TODO create weights vector
        }
    }


    pub fn compute(&self, input: &[f64]) -> f64{
        let mut net_input:  f64 = 0.0;
        for i in 0..input.len() {
            net_input += self.weights[i] * input[i];
        }
        net_input -= self.threshold;
        if net_input >= 0.0 {
            return 1.0;
        }
        0.0
    }

    pub fn learn(&mut self, vector_input: &[f64], good_answer: f64) {
        let output = self.compute(vector_input);
        let error = good_answer - output;
        for i in 0..self.weights.len() {
            self.weights[i] += self.alfa * error * vector_input[i];
        }
        self.threshold -= self.alfa * error;
    }
}