use std::io;
use std::fs::File;
use std::io::Read;
use std::mem;
use random::Source;

println!("Scalar: {:?}", );
println!("Vector: {:?}", );

struct Neuron {
    weight: Vec<f64>,
    biai: f64,
    activation_func: fn (f64) -> f64,
    pub activation: f64,
}

impl Neuron {
    fn new_random(nb_conn: u64, activation_func: fn (f64) -> f64) -> Self {
        let mut source = random::default().seed([42, 69]);
        Neuron {
            weight: source.iter().take(nb_conn).collect(),
            biai: source.read(),
            activation: 0.0,
            activation_func,
        }
    }
    fn calc_activation(&mut self, layer: &Vec<Neuron>) {
        self.activation = (self.activation_func)(layer.iter().zip(&self.weight).fold(self.biai, |acc, (n, w)| acc + n.activation * w));
    }
}

struct NeuralNetwork {
    layer: Vec<Vec<Neuron>>,
}

impl NeuralNetwork {
    fn new(layer_size: Vec<u32>) -> Self {
        NeuralNetwork {
            layer: layer_size.iter().map(|s| 0..s.map(|_| Neuron::new_random()).collect()).collect()
        }
    }
    /// compute the neural network
    /// take an image and return un tuple with the number guessed and the 
    /// probability
    fn compute(&mut self, img: &Image) -> (u8, f64) {
        assert_eq!(img.data.len(), self.layer.len());

        // initialise first layer with pixels
        for (i, px) in img.data.iter().enumerate() {
            self.layer[0][i].activation = *px as f64 / 256.0;
        }

        for (i, l) in self.layer.iter_mut().skip(1).enumerate() {
            for n in &mut l {
                n.calc_activation(&l[i - 1]);
            }
        }
        let (res, prob) = self.layer.last().iter().enumerate().max_by(|(_, n1), (_, n2)| n1.activation >= n2.activation);
    }
}

fn parse_label_file(filename: &str) -> io::Result<Vec<u8>> {
    let mut file = File::open(filename)?;
    let res = file.bytes().skip(8).collect::<io::Result<Vec<u8>>>()?;
    Ok(res)
}

fn parse_image_file(filename: &str) -> io::Result<Vec<Image>> {
    let mut file = File::open(filename)?;
    let mut magic = [0; 4];
    file.read_exact(&mut magic)?;
    unsafe {
        let magic_nb = mem::transmute::<[u8; 4], u32>(magic);
        println!("{:?}", u32::from_le(magic_nb));
    }
    let mut nbr_image = [0; 4];
    file.read_exact(&mut nbr_image)?;
    let mut nb_image;
    unsafe {
        nb_image = u32::from_be(mem::transmute::<[u8; 4], u32>(nbr_image));
        println!("{:?}", nb_image);
    }
    file.read_exact(&mut nbr_image)?; //TODO: get colon and line nbr
    file.read_exact(&mut nbr_image)?;
    let mut res = Vec::with_capacity(nb_image as usize);
    for i in 0..nb_image {
        let mut buffer = [0; 28 * 28];
        file.read_exact(&mut buffer)?;
        res.push(Image::new(buffer.to_vec()));
    }
    //let images = res.split_off(16);
    Ok(res)
}

struct Image {
    data: Vec<u8>,
}

impl Image {
    fn new(data: Vec<u8>) -> Self {
        Image {
            data,
        }
    }
}

use std::fmt::Display;
use std::fmt;

impl Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let n = 28;
        let s = self.data
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, &nb)| {
                if i % n == 0 {
                    if nb > 0 {
                        format!("{}\n#", acc)
                    }
                    else {
                        format!("{}\n.", acc)
                    }
                } else {
                    if nb > 0 {
                        format!("{}#", acc)
                    }
                    else {
                        format!("{}.", acc)
                    }
                }
            });

        write!(f, "({}\n {})", n, s)
    }
}

fn main() {
    let mut args = std::env::args().skip(1);
    let labels = parse_label_file(&args.next().unwrap()).unwrap();
    let images = parse_image_file(&args.next().unwrap()).unwrap();
    //println!("{:?}", labels);
    println!("{}", images[7]);
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    #[test]
    fn activation() {
    }
}

