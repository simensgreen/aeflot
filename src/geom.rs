use std::collections::HashMap;
use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;

#[derive(Debug)]
pub struct OldFile{
    pub name: String,
    pub j: [i8; 7],  // Все переменные J<n>
    nwaf: usize,
    nwafor: isize,
    nfus: u8,
    nradx_1: usize,
    nforx_1: usize,
    nradx_2: usize,
    nforx_2: usize,
    nradx_3: usize,
    nforx_3: usize,
    nradx_4: usize,
    nforx_4: usize,
    np: usize,
    kfield: bool,
    nf: u8,
    nfinor: usize,
    ncan: u8,
    ncanor: isize,
    itemax: bool,
    ground: bool,
    bet: bool,
    diver: bool,
    beloyc: bool,
    shek: bool,
    npodor: [i8; 9],
    npusor: [i8; 9],
    npradx: [i8; 9],
    kfx: i8,
    kyf: i8,
    wing_data: Vec<[f64; 4]>,
}

impl OldFile{
    pub fn new() -> OldFile {
        OldFile{
            name:String::new(),
            j: [0; 7],
            nwaf: 0,
            nwafor: 0,
            nfus: 0,
            nradx_1: 0,
            nforx_1: 0,
            nradx_2: 0,
            nforx_2: 0,
            nradx_3: 0,
            nforx_3: 0,
            nradx_4: 0,
            nforx_4: 0,
            np: 0,
            kfield: false,
            nf: 0,
            nfinor: 0,
            ncan: 0,
            ncanor: 0,
            itemax: false,
            ground: false,
            bet: false,
            diver: false,
            beloyc: false,
            shek: false,
            npodor: [0; 9],
            npusor: [2; 9],
            npradx: [2; 9],
            kfx: 0,
            kyf: 0,
            wing_data: vec![]
        }
    }

    pub fn read(path: &str) -> OldFile {
        let mut new_file = OldFile::new();
        let fs_file = File::open(path).unwrap();
        new_file.parse(fs_file);
        new_file
    }

    fn parse(&mut self, file: File){
        for (line_no, result_line) in BufReader::new(file).lines().enumerate(){
            let line = result_line.unwrap();
            match line_no {
                0 => self.parse_name(line),
                1 => self.parse_2_line(line),
                2 => self.parse_3_line(line),
                3 => self.parse_4_line(line),
                _ => {
                    // print!("line_no: {} {} {}", line_no, (line_no > 7), (line_no < 8 + self.nwaf));
                    if (line_no > 6) && (line_no < 7 + self.nwaf) {
                        self.parse_wing_data(line)
                    }
                    else {
                        println!("unparsed line №{}: {}", line_no, line)
                    }
                }
            }
        }
    }

    fn parse_name(&mut self, line: String) {
        let name = line.trim();
        self.name = String::from(name);
    }

    fn parse_2_line(&mut self, line: String) {
        for (no, number) in line.split_whitespace().enumerate(){
            match no {
                0..=6 => self.j[no] = i8::from_str_radix(number, 10).unwrap(),
                7 => {
                    let pair: Vec<&str> = number.split("-").collect();
                    self.nwaf = usize::from_str_radix(pair[0], 10).unwrap();
                    self.wing_data.reserve(self.nwaf);
                },
                _ => todo!() // Разобраться с этими переменными (например в TU204Z "16-29")
                // 16 это nwaf
            }
        }
    }

    fn parse_3_line(&mut self, line:String) {
        for (no, value) in line.split_whitespace().enumerate() {
            match no {
                0 => self.itemax = string_to_bool(value),
                1 => self.ground = string_to_bool(value),
                2 => self.bet = string_to_bool(value),
                3 => self.diver = string_to_bool(value),
                4 => self.beloyc = string_to_bool(value),
                5 => self.shek = string_to_bool(value),
                _ => unreachable!()
            }
        }
    }

    fn parse_4_line(&mut self, line: String){

    }

    fn parse_wing_data(&mut self, line: String) {
        let str_coords: Vec<&str> = line.split_whitespace().collect();
        let mut coords: [f64; 4] = [0.0; 4];
        for (no, coord) in str_coords.iter().enumerate(){
            coords[no] = f64::from_str(coord).unwrap()
        }
        self.wing_data.push(coords);
    }
}

fn string_to_bool(string: &str) -> bool {
    !((string.to_lowercase() == "false") || (string == "0"))
}



// fn geom(){
// }
