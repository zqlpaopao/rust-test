use rand::Rng;

// https://mp.weixin.qq.com/s/wnmdbF9hYFq55veusS7Fdg

//定义神经网络单层结构
struct Layer{
    weights : Vec<Vec<f64>>,
    biases : Vec<f64>
}

impl Layer {
    fn new(input_size : usize,output_size : usize) -> Layer {
        let mut rng = rand::thread_rng();

        //用随机值初始化权重和偏置
        let weights = (0..output_size)
            .map(|_|(0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();

        let biases = (0..output_size)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect::<Vec<f64>>();

        Layer{weights,biases}
    }
}

//前向传播

//Sigmoid 激活函数
fn sigmoid(x:f64)->f64{
    1.0 / (1.0+(-x).exp())
}

impl Layer{
    fn forward(&self,input : &[f64])->Vec<f64>{
        self.weights
            //用于遍历集合的不可变引用。
            .iter()
            //为迭代器添加索引。
            .enumerate()
            .map(|(i,neuron_weights)|{
                let sum : f64 = neuron_weights.iter().zip(input.iter())
                    .map(|(w,i) |w*i)
                    .sum();
                sigmoid(sum+self.biases[i])
            }).collect()
    }
}

//定义神经网络
struct NeuralNetwork{
    layers : Vec<Layer>,
}

impl NeuralNetwork{
    fn new(layer_sizes: &[usize]) -> NeuralNetwork{
        let layers = layer_sizes.windows(2)
            .map(|w|Layer::new(w[0], w[1]))
            .collect();
        NeuralNetwork{layers}
    }

    fn forward(&self,input: &[f64])->Vec<f64>{
        self.layers.
            //便利不可变引用
            iter().
            //从初始值开始，依次将每一层的结果累积到 acc 中。
            // 2. fold(input.to_vec(), ...)
            // fold 方法接受两个参数：
            //
            // 初始值：input.to_vec()，将输入数据转换为向量（Vec<f64>）。
            //
            // 闭包：定义如何累积结果。
            //
            // fold 的工作方式：
            //
            // 从初始值开始，依次将每一层的结果累积到 acc 中。
            //
            // 3. 闭包 |mut acc, layer| { ... }
            // acc：累积值（当前层的输入）。
            //
            // layer：当前层。
            //
            // mut acc：acc 是可变的，因为每一层的输出会更新 acc。
            //
            // 4. layer.forward(&acc)
            // 调用当前层的 forward 方法，传入 acc 作为输入。
            //
            // 返回当前层的输出，作为下一层的输入。
            fold(input.to_vec(),| acc,layer|{layer.forward(&acc)})
    }
}

//计算损失loss
fn mean_squared_error(predicted:&[f64], actual:&[f64])->f64{
    predicted.iter()
        .zip(actual.iter())
        .map(|(p, a)| (p - a).powi(2))
        .sum::<f64>() / predicted.len() as f64
}


//实现反向传播
impl Layer {
    fn backward(&mut self,input:&[f64],error : &[f64],learning_rate:f64)->Vec<f64>{
        let mut input_error = vec![0.0; input.len()];

        for (i, neuron_weights) in self.weights.iter_mut().enumerate() {
            for (j, weight) in neuron_weights.iter_mut().enumerate() {
                input_error[j] += *weight * error[i];
                *weight -= learning_rate * error[i] * input[j];
            }
            self.biases[i] -= learning_rate * error[i];
        }

        input_error
    }
}

impl NeuralNetwork {
    fn backward(&mut self, inputs: &[f64], target: &[f64], learning_rate: f64) {
        let mut layer_inputs = vec![inputs.to_vec()];
        let mut current_input = inputs.to_vec();

        for layer in &self.layers {
            current_input = layer.forward(&current_input);
            layer_inputs.push(current_input.clone());
        }

        let error = layer_inputs.last().unwrap()
            .iter()
            .zip(target.iter())
            .map(|(o, t)| o - t)
            .collect::<Vec<_>>();

        let mut current_error = error;

        for (layer, inputs) in self.layers.iter_mut().rev().zip(layer_inputs.iter().rev().skip(1)) {
            current_error = layer.backward(inputs, &current_error, learning_rate);
        }
    }
}


pub fn test_network(){
    let mut network = NeuralNetwork::new(&[2, 3, 1]);
    let data = vec![
        (vec![0.0, 0.0], vec![0.0]),
        (vec![0.0, 1.0], vec![1.0]),
        (vec![1.0, 0.0], vec![1.0]),
        (vec![1.0, 1.0], vec![0.0]),
    ];

    let learning_rate = 0.1;
    for epoch in 0..5000 {
        let mut loss = 0.0;

        for (input, target) in &data {
            let prediction = network.forward(input);
            loss += mean_squared_error(&prediction, target);
            network.backward(input, target, learning_rate);
        }

        if epoch % 1000 == 0 {
            println!("Epoch {}: Loss = {}", epoch, loss / data.len() as f64);
        }
    }

}