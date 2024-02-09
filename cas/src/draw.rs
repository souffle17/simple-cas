use std::io::{self, Write};

use crate::compute::NumberNode;

pub fn point_check(left_expression: Option<&NumberNode>, right_expression: Option<&NumberNode>, x: f64, y: f64, x_scale: f64, y_scale: f64) -> bool {

    let x_scale = x_scale * 0.4;
    let y_scale = y_scale * 0.4;
    if left_expression.is_none() || right_expression.is_none() {
        false
    }
    else {

        let mut fov: [(f64, f64); 9] = [(f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN), 
            (f64::NAN, f64::NAN)
        ];

        let mut index = 0;
        for i in -1..2 {
            for j in -1..2 {
                fov[index] = (left_expression.unwrap().resolve(&(x + (j as f64*x_scale)), &(y + (i as f64*y_scale))), 
                    right_expression.unwrap().resolve(&(x + (j as f64*x_scale)), &(y + (i as f64*y_scale))));
                index += 1;
            }
        }
        
        let mut pass: bool = false;

        // check for intersection
        for n in 0..4 {
            pass = pass || (
                (fov[n].0 - fov[n].1 > 0.0) != (fov[8-n].0 - fov[8-n].1 > 0.0)
            );
        }

        // not nan
        pass = pass && !(left_expression.unwrap().resolve(&x, &y).is_nan() || right_expression.unwrap().resolve(&x, &y).is_nan());

        pass
    }

}

pub fn make_graph(left_side: Option<&NumberNode>, right_side: Option<&NumberNode>, parameters: Vec<String>) -> String {
    let x_min_in = str::parse::<f64>(&parameters[0]);
    let x_max_in = str::parse::<f64>(&parameters[1]);
    let y_min_in = str::parse::<f64>(&parameters[2]);
    let y_max_in = str::parse::<f64>(&parameters[3]);
    let x_scale_in = str::parse::<f64>(&parameters[4]);
    let y_scale_in = str::parse::<f64>(&parameters[5]);

    
    let x_scale = match x_scale_in {
        Ok(_) => x_scale_in.unwrap(),
        Err(_) => {
            eprintln!("invalid x scale, using default");
            1.0
        },
    };
    
    let y_scale = match y_scale_in {
        Ok(_) => y_scale_in.unwrap(),
        Err(_) => {
            eprintln!("invalid y scale, using default");
            1.0
        },
    };

    let x_min = match x_min_in {
        Ok(_) => (x_min_in.unwrap() / x_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid x min, using default"); 
            -10
        },
    };
    
    let x_max = match x_max_in {
        Ok(_) => (x_max_in.unwrap() / x_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid x max, using default"); 
            10
        },
    };

    let y_min = match y_min_in {
        Ok(_) => (y_min_in.unwrap() / y_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid y min, using default"); 
            -10
        },
    };
    
    let y_max = match y_max_in {
        Ok(_) => (y_max_in.unwrap() / y_scale).round() as i64,
        Err(_) => {
            eprintln!("invalid y max, using default"); 
            10
        },
    };

    let mut graph: Vec<Vec<char>> = Vec::new();

    //coordinate axis
    for i in y_min..y_max + 1 {
        let mut row: Vec<char> = Vec::new();
        for j in x_min..x_max + 1 {
            if i != 0 {
                if j != 0 {
                    row.push(' ');
                } else {
                    row.push('|');
                }
            } else if j == 0 {
                row.push('+');
            } else {
                row.push('-');
            }
        }
        graph.push(row);
    }
    
    if left_side.is_none() || right_side.is_none() {
        if left_side.is_none() {
            eprintln!("first expression couldn't be resolved");
        }
        if right_side.is_none() {
            eprintln!("second expression couldn't be resolved");
        }

        "".to_string()
    }
    else {
        let mut output: String = String::new();

        for i in (y_min..(y_max + 1)).rev() {
            for j in x_min..(x_max + 1) {
                if point_check(left_side, right_side, j as f64 * x_scale, i as f64 * y_scale,  x_scale, y_scale) {
                    graph[(i - y_min) as usize][(j - x_min) as usize] = '•';
                }
            }

            for char in &graph[(i - y_min) as usize] {
                output.push(*char);
            }
    
            output.push('\n');

        }

        output
    }
}

pub fn draw_prompt(left_expression: Option<&NumberNode>, right_expression: Option<&NumberNode>) {
    let mut parameters: Vec<String> = Vec::new();

    let mut input: String = "".to_string();
    
    print!("x-axis minimum (default -10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "-10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("x-axis maximum (default 10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("y-axis minimum (default -10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "-10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("y-axis maximum (default 10): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "10".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("x scale (default 1.0): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "1.0".to_string();
    }

    parameters.push(input.clone());
    input = "".to_string();

    print!("y scale (default 1.0): ");
    let _ = io::stdout().flush();

    let _ = io::stdin().read_line(&mut input).is_ok();

    if input.len() == 2 {
        input = "1.0".to_string();
    }

    parameters.push(input.clone());

    for s in &mut parameters {
        *s = s.trim_end().to_string();
    }

    println!();
    println!("{}", make_graph(left_expression, right_expression, parameters));
}