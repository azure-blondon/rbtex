use image::Rgba;




#[derive(Debug, Clone, PartialEq)]
pub enum BiOSToken {
    Char(char),
    Instant(String),
    ColorStart(Rgba<u8>),
    Pause(usize),
    Newline,
}


fn parse_color(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> Rgba<u8> {
    let mut color_str: String = String::new();
    let color: Rgba<u8>;
    
    while let Some(c) = chars.next() {
        if c == '§' {
            break;
        }
        color_str.push(c);
    }
    if color_str.contains(',') {
        let mut color_vec: Vec<u8> = color_str.split(',').map(|x| x.parse::<u8>().expect("Invalid color")).collect();
        if color_vec.len() == 3 {
            color_vec.push(255);
        }
        assert!(color_vec.len() == 4, "Color has a wrong number of channels (either 3 or 4)");

        color = Rgba([*color_vec.get(0).unwrap(), *color_vec.get(1).unwrap(), *color_vec.get(2).unwrap(), *color_vec.get(3).unwrap()]);
    } else {
        color = match color_str.as_str() {
            "reset" => Rgba([255, 255, 255, 255]),
            "red"   => Rgba([255, 0, 0, 255]),
            "green" => Rgba([0, 255, 0, 255]),
            "blue"  => Rgba([0, 0, 255, 255]),
            _ => panic!("Invalid color"),
        };
        


    }
    
    
    color
}

fn parse_usize(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> usize {
    let mut amount_str: String = String::new();
    let amount: usize;
    
    while let Some(c) = chars.next() {
        if c == '§' {
            break;
        }
        amount_str.push(c);
    }
    amount = amount_str.parse::<usize>().expect("Invalid number");

    amount
}


fn parse_str(chars: &mut std::iter::Peekable<std::str::Chars<'_>>) -> String {
    let mut instant_string: String = String::new();
    
    while let Some(c) = chars.next() {
        if c == '§' {
            break;
        }
        instant_string.push(c);
    }

    instant_string
}


pub fn parse_bios_string(bios_string: &str) -> Vec<BiOSToken> {
    let mut tokens: Vec<BiOSToken> = Vec::new();
    let mut chars: std::iter::Peekable<std::str::Chars<'_>> = bios_string.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '§' => {
                if let Some(command_type) = chars.peek() {
                    match command_type {
                        'c' => {
                            chars.next(); // c
                            chars.next(); // :
                            let color: Rgba<u8> = parse_color(&mut chars);
                            tokens.push(BiOSToken::ColorStart(color));
                        }
                        'p' => {
                            chars.next(); // p
                            chars.next(); // :
                            let amount: usize = parse_usize(&mut chars);
                            tokens.push(BiOSToken::Pause(amount));
                        }
                        'i' => {
                            chars.next(); // i
                            chars.next(); // :
                            let instant_string: String = parse_str(&mut chars);
                            tokens.push(BiOSToken::Instant(instant_string));
                        }
                        _ => tokens.push(BiOSToken::Char(c))
                    }
                } else {
                    tokens.push(BiOSToken::Char(c));
                }
            }
            '\n' => tokens.push(BiOSToken::Newline),
            _ => tokens.push(BiOSToken::Char(c))
        }
    }

    tokens
}

