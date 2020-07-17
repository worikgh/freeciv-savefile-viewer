use std::env;
use regex::Regex;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::path;
// use std::io::Read;

// To decode save game files
use xz2::read::XzDecoder;

fn read_save_file(file_name: &String) -> io::Result<String> {
    let path = path::Path::new(file_name.as_str());
    let mut file = File::open(&path)?;
    let mut buffer = Vec::new();
    // read the whole file
    file.read_to_end(&mut buffer)?;

    let mut decompressor = XzDecoder::new(&buffer[..]);
    let mut contents = String::new();
    decompressor.read_to_string(&mut contents).unwrap();
    Ok(contents)
}

/// Return the SVG string defining a hexagon
fn hexagon(centrex:f64, centrey:f64, big_r:f64,
	   r:f64, fill:&str, stroke_width:usize) -> String
{
    let x1 = centrex - big_r;
    let y1 = centrey;

    let x2 = centrex- big_r*0.5;
    let y2 = centrey + r;
    
    let x3 = centrex + big_r*0.5;
    let y3 = y2;

    let x4 = centrex + big_r;
    let y4 = centrey;

    let x5 = x3;
    let y5 = centrey - r;

    let x6 = x2;
    let y6 = y5;

    format!("<polyline points='{}, {} {}, {} {}, {} {}, {} {}, {} {}, {} {}, {}' stroke='black' fill='{}' stroke-width='{}'/>\n",
	    x1, y1, x2, y2, x3, y3, x4, y4, x5, y5, x6, y6, x1, y1,
	    fill, stroke_width)
}


fn svg_city(x:f64, y:f64, big_r:f64, colour:&String) -> String {
    format!("<circle  cx='{}' cy='{}' r='{}'  stroke='black' fill='{}' stroke-width='1'/>\n",
	    x, y, big_r, colour)
}
fn main() -> io::Result<()> {

    let args: Vec<String> = env::args().collect();
    let arg = &args[1];
    // let text = read_save_file(arg).unwrap();
    // let lines = text.as_str().split("\n");
    // Lines is a iterator over lines in the original file

    // Use a state machine to process the file
    #[derive(PartialEq)]
    enum State {
	Initial,
	Map,
	Settings,
	Player,
	Units,
	Cities,
    };
    let mut state = State::Initial;

    // Variables used in parsing

    // Describe a player's data
    // let mut player_n:usize; // Player number
    let mut colour_r:usize = 0;
    let mut colour_g:usize = 0;
    let mut colour_b:usize = 0;

    // Regular expressions for parsing the file
    let re_player_n = Regex::new(r"\[player(\d+)\]$").unwrap();
    let re_terrain =  Regex::new(r"t\d{4}=.(.+).$").unwrap();
    let re_xysize = Regex::new(r"([xy])size.,(\d+),(\d+)$").unwrap();
    let re_cities = Regex::new(r"^dc=").unwrap();
    let re_units = Regex::new(r"^u=").unwrap();
    let re_colour = Regex::new(r"^color.([rgb])=(\d+)$").unwrap();
    
    let mut terrain_rows = Vec::new();
    let mut units = Vec::new();
    let mut cities = Vec::new();
    
    let mut xsize:usize = 0;
    let mut ysize:usize = 0;

    let contents = read_save_file(arg).unwrap(); //let file = File::open(arg.as_str())?;
    for line in contents.lines() {
	// Each line of the save file
	let line = line.to_string();
	println!("{}", line);
	if line == "" {
	    state = State::Initial;
	    continue;
	}
	match state {
	    State::Initial => {
		if line == "[map]" {
		    state = State::Map;
		    continue;
		}else if line == "[settings]"{
		    state = State::Settings;
		    continue;
		}else if re_player_n.is_match(line.as_str()) {
		    // let cap = re_player_n.captures(line).unwrap();
		    // player_n =
		    // 	cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
		    state = State::Player;
		    continue;
		}
	    },
	    State::Map => {
		if re_terrain.is_match(line.as_str()) {
		    let cap = re_terrain.captures(line.as_str()).unwrap();
		    let row:Vec<char> = cap.get(1).unwrap().as_str().split("").
			filter(|x| x.len() == 1).
			map(|x| {
			    assert!(x.len() == 1);
			    x.chars().next().unwrap()
			}).collect();
		    terrain_rows.push(row);
		}
	    },

	    State::Settings=> {
		if re_xysize.is_match(line.as_str()){
		    let cap = re_xysize.captures(line.as_str()).unwrap();
 		    assert!(cap.get(2).unwrap().as_str().
			    parse::<usize>().unwrap()
			    ==
			    cap.get(3).unwrap().as_str().
			    parse::<usize>().unwrap());
		    match cap.get(1).unwrap().as_str() {
			"x" => xsize =
			    cap.get(2).unwrap().as_str().
			    parse::<usize>().unwrap(),
			"y" => ysize =
			    cap.get(2).unwrap().as_str().
			    parse::<usize>().unwrap(),
			_ => panic!(line),
		    };
		}
	    },
	    State::Player=> {
		if re_units.is_match(line.as_str()){
		    state= State::Units;
		    continue;
		}else if re_cities.is_match(line.as_str()) {
		    state= State::Cities;
		    continue;
		}else if re_colour.is_match(line.as_str()) {
		    let cap = re_colour.captures(line.as_str()).unwrap();
		    let colour = cap.get(2).unwrap().as_str().
			parse::<usize>().unwrap();
		    match cap.get(1).unwrap().as_str() {
			"r" => colour_r = colour,
			"g" => colour_g = colour,
			"b" => colour_b = colour,
			_ => panic!(line),
		    };
		    
		}
		
	    },
	    State::Units=> {
		if line == "}" {
		    state = State::Player;
		    continue;
		}
		let v:Vec<String> = line.split(",").map(|x| x.to_string()).collect();
		assert!(v.len() > 9);
		units.push((v[1].clone(), v[2].clone(),
			    format!("rgb({}, {}, {})",
				    colour_r, colour_g, colour_b),
			    v[8].clone()));
	    },
	    State::Cities=> {
		if line == "}" {
		    state = State::Player;
		    continue;
		}
		let v:Vec<String> = line.split(",").
		    map(|x| x.to_string()).collect();
		assert!(v.len() > 1);
		cities.push((v[1].parse::<usize>().unwrap(), // x
			     v[0].parse::<usize>().unwrap(), // y
			     format!("rgb({}, {}, {})",
				     colour_r, colour_g, colour_b)));
	    }
	}
    }


    // println!("With text:\n{}", text);    
    // let scale = 6;
    let big_r:f64 = 28.0;
    let r = big_r * 30.0_f64.to_radians().cos();
    let repeatx = xsize;
    let repeaty = ysize;
    let stroke_width = 2;
     
    let maxx = 4.0 * repeatx as f64 * big_r;
    let maxy = 2.5 * repeaty as f64 * r;
    let mut odd = false;
    let mut y = r;
    let terrain_colour = [('i', "black"), // Inaccessible
			  ('+', "#848ce9"), // Lake
			  (' ', "#3e46a7"), // Ocean
			  (':', "#060d69"), // Deep Ocean
			  ('a', "white"), // Glacier
			  ('d', "#f0e7a1"), // Desert
			  ('f', "#039c14"), // Forest
			  ('g', "#15d42b"), // Grassland
			  ('h', "#2a7c34"), // Hills
			  ('j', "#056710"), // Jungle
			  ('m', "grey"), // Mountains
			  ('p', "#078315"), // Plains
			  ('s', "#a0c3a4"), // Swamp
			  ('t', "#3a4b04"), // Tundra
    ];
    let mut svg = format!("<svg version='1.1' width='{}' height='{}' xmlns='http://www.w3.org/2000/svg'>\n", maxx, maxy);
    for row in terrain_rows.iter(){
	let mut x = match odd {
	    true => big_r*2.5,
	    false => big_r,
	};
	odd = !odd;
	if y >= maxy {
	    break;
	}
	for t in row.iter() {
	    if x >= maxx {
		break;
	    }
	    let fill = terrain_colour.iter().
		filter(|_t| &_t.0 == t).
		next().unwrap().1;
	    let h = hexagon(x, y, big_r, r, fill, stroke_width);
	    svg += h.as_str();
	    x += 3.0*big_r;
	}
	y += r;
    }
    for c in cities {

	// The x/y coordinates in game space and the players colour
	let x = c.0;
	let y = c.1;
	let colour = c.2;

	// Convert coordinates to hexagon/SVG space
	let x = if x % 2 == 1 {
	    // x is odd
	    big_r * 2.5
	}else{
	    big_r
	} + x as f64 * 3.0  * big_r;
	let y = (y as f64 + 1.0) * r;

	// Get svg city
	let city = svg_city(x, y, big_r, &colour);
	svg += city.as_str();
    }


    svg += "</svg>\n";
    
    let path = path::Path::new(arg.as_str());    
    let path = path.with_extension("svg");
    let mut file = File::create(path.as_path().to_str().unwrap())?;
    file.write_all(svg.as_bytes())?;
    Ok(())
}
