use rand::Rng;

// 定义神经网络单层结构
struct Layer {
    weights: Vec<Vec<f64>>,
    biases: Vec<f64>,
}

impl Layer {
    fn new(input_size: usize, output_size: usize) -> Layer {
        let mut rng = rand::thread_rng();
        let weights = (0..output_size)
            .map(|_| (0..input_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();
        let biases = (0..output_size)
            .map(|_| rng.gen_range(-1.0..1.0))
            .collect::<Vec<f64>>();
        Layer { weights, biases }
    }

    fn forward(&self, input: &[f64]) -> Vec<f64> {
        self.weights
            .iter()
            .enumerate()
            .map(|(i, neuron_weights)| {
                let sum: f64 = neuron_weights
                    .iter()
                    .zip(input.iter())
                    .map(|(w, i)| w * i)
                    .sum();
                sigmoid(sum + self.biases[i])
            })
            .collect()
    }

    fn backward(&mut self, input: &[f64], error: &[f64], learning_rate: f64) -> Vec<f64> {
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

// 定义神经网络
struct NeuralNetwork {
    layers: Vec<Layer>,
}

impl NeuralNetwork {
    fn new(layer_sizes: &[usize]) -> NeuralNetwork {
        let layers = layer_sizes
            .windows(2)
            .map(|w| Layer::new(w[0], w[1]))
            .collect();
        NeuralNetwork { layers }
    }

    fn forward(&self, input: &[f64]) -> Vec<f64> {
        self.layers
            .iter()
            .fold(input.to_vec(), |acc, layer| layer.forward(&acc))
    }

    fn backward(&mut self, inputs: &[f64], target: &[f64], learning_rate: f64) {
        let mut layer_inputs = vec![inputs.to_vec()];
        let mut current_input = inputs.to_vec();
        for layer in &self.layers {
            current_input = layer.forward(&current_input);
            layer_inputs.push(current_input.clone());
        }
        let error = layer_inputs
            .last()
            .unwrap()
            .iter()
            .zip(target.iter())
            .map(|(o, t)| o - t)
            .collect::<Vec<_>>();
        let mut current_error = error;
        for (layer, inputs) in self
            .layers
            .iter_mut()
            .rev()
            .zip(layer_inputs.iter().rev().skip(1))
        {
            current_error = layer.backward(inputs, &current_error, learning_rate);
        }
    }
}

// Sigmoid 激活函数
fn sigmoid(x: f64) -> f64 {
    1.0 / (1.0 + (-x).exp())
}

// 计算损失
fn mean_squared_error(predicted: &[f64], actual: &[f64]) -> f64 {
    predicted
        .iter()
        .zip(actual.iter())
        .map(|(p, a)| (p - a).powi(2))
        .sum::<f64>()
        / predicted.len() as f64
}

// 滑动窗口生成
fn create_sliding_windows(data: &[f64], window_size: usize) -> Vec<Vec<f64>> {
    data.windows(window_size)
        .map(|window| window.to_vec())
        .collect()
}

// 训练模型
fn train_model(
    network: &mut NeuralNetwork,
    data: &[Vec<f64>],
    targets: &[Vec<f64>],
    learning_rate: f64,
    epochs: usize,
) {
    for epoch in 0..epochs {
        let mut loss = 0.0;
        for (input, target) in data.iter().zip(targets.iter()) {
            let prediction = network.forward(input);
            loss += mean_squared_error(&prediction, target);
            network.backward(input, target, learning_rate);
        }
        if epoch % 1000 == 0 {
            println!("Epoch {}: Loss = {}", epoch, loss / data.len() as f64);
        }
    }
}

// 异常检测
fn is_anomaly(predicted: &[f64], actual: &[f64], threshold: f64) -> bool {
    let error = mean_squared_error(predicted, actual);
    error > threshold
}

// 主函数
pub fn test_stream() {
    // 示例数据
    let data = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
    let window_size = 3;
    let windows = create_sliding_windows(&data, window_size);

    // 目标值（下一个时间点的值）
    let targets = data[window_size..]
        .iter()
        .map(|&x| vec![x])
        .collect::<Vec<Vec<f64>>>();

    // 创建神经网络
    let mut network = NeuralNetwork::new(&[window_size, 5, 1]);

    // 训练模型
    train_model(&mut network, &windows, &targets, 0.1, 10000);

    // 实时预测与异常检测
    let new_data = vec![0.8, 0.9, 1.0];
    let prediction = network.forward(&new_data);
    let actual = vec![100.1];
    let threshold = 0.01;
    if is_anomaly(&prediction, &actual, threshold) {
        println!("Anomaly detected!");
    } else {
        println!("No anomaly.");
    }
}
