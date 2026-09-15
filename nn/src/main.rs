fn main(){
	let inputs = [1.0,2.0,3.0];
	let weights = [0.5, -0.3, 0.8];
	let bias = 0.2;
	let mut output = bias;
	for i in 0..inputs.len(){
		output += inputs[i]*weights[i];
	}
	println!("outputs = {}", output);
}
