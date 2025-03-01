fn main() {
  let formsA: [f32; 16] = ["Deep Sea", "Lakes", "Rivers", "Coasts", "Wetlands", "Desert", "Plains", "Forest", "Caves", "Tundra", "Valley", "Mountains", "Rock Formations", "Weather", "Atmospheric Optical Phenomenona", "Astronomy"];
  let formsB: [f32; 16] = [5.9, 6.5, 7.1, 7.7, 4.1, 2.9, 8.9, 9.5, 4.7, 2.3, 8.3, 5.3, 3.5, 1.7, 0.5, 1.1];
  let successionsA: [f32; 16] = ["fires", "floods", "droughts", "windstorms", "landslides", "avalanches", "volcanic eruptions", "disease epidemics", "ocean temperature changes"];
  let successionsB: [f32; 16] = [2.0, 5.0, 3.0, 6.0, 7.0, 9.0, 1.0, 8.0, 4.0];
  
  println!("Rate forms by frequency from 0.0 to 10.0:");
  println!("Deep Sea");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let a: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Lakes");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let b: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Rivers");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let c: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Coasts");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let d: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Wetlands");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let e: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Desert");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let f: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Plains");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let g: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Forest");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let h: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Caves");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let i: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Tundra");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let j: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Valley");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let k: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Mountains");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let l: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Rock Formations");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let m: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Weather");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let n: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Atmospheric Optical Phenomenona");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let o: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("Astronomy");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let p: f32 = input_line.trim().parse().expect("Input not an integer");

  println!("Rate succession frequencies:");
  println!("fires");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let q: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("floods");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let r: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("droughts");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let s: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("windstorms");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let t: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("landslides");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let u: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("avalanches");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let v: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("volcanic eruptions");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let w: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("disease epidemics");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let x: f32 = input_line.trim().parse().expect("Input not an integer");
  println!("ocean temperature changes");
  let mut input_line = String::new();
  io::stdin().read_line(&mut input_line).expect("Failed to read line");
  let y: f32 = input_line.trim().parse().expect("Input not an integer");

  let aa: f32 = ((5.9*a)+(6.5*b)+(7.1*c)+(7.7*d)+(4.1*e)+(2.9*f)+(8.9*g)+(9.5*h)+(4.7*i)+(2.3*j)+(8.3*k)+(5.3*l)+(3.5*m)+(1.7*n)+(0.5*o)+(1.1*p))/160;
  let bb: f32 = ((2.0*q)+(5.0*r)+(3.0*s)+(6.0*t)+(7.0*u)+(9.0*v)+(1.0*w)+(8.0*x)+(4.0*y))/90;
  let z: f32 = aa*bb;
  println!("Environment Score "+z);
}
