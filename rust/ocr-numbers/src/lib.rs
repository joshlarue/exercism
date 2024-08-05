// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

#[derive(Debug)]
pub enum KeyType {
    Pipe,
    Underscore,
    Space,
    Invalid,
}

impl KeyType {
    fn new(key: char) -> KeyType {
        match key {
            '|' => KeyType::Pipe,
            '_' => KeyType::Underscore,
            ' ' => KeyType::Space,
            _ => {
                eprintln!("Uh oh! Invalid keytype.");
                KeyType::Invalid
            }
        }
    }
}

impl PartialEq for KeyType {
    fn eq(&self, other: &KeyType) -> bool {
        match (self, other) {
            (KeyType::Pipe, KeyType::Pipe) => true,
            (KeyType::Underscore, KeyType::Underscore) => true,
            (KeyType::Space, KeyType::Space) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub struct Number {
    top_left: Option<KeyType>,
    top_middle: Option<KeyType>,
    top_right: Option<KeyType>,
    upper_middle_left: Option<KeyType>,
    upper_middle_middle: Option<KeyType>,
    upper_middle_right: Option<KeyType>,
    lower_middle_left: Option<KeyType>,
    lower_middle_middle: Option<KeyType>,
    lower_middle_right: Option<KeyType>,
    bottom_left: Option<KeyType>,
    bottom_middle: Option<KeyType>,
    bottom_right: Option<KeyType>,
}

impl Number {
    fn new() -> Number {
        Number {
            top_left: None,
            top_middle: None,
            top_right: None,
            upper_middle_left: None,
            upper_middle_middle: None,
            upper_middle_right: None,
            lower_middle_left: None,
            lower_middle_middle: None,
            lower_middle_right: None,
            bottom_left: None,
            bottom_middle: None,
            bottom_right: None,
        }
    }

    fn update(&mut self, index: usize, key: KeyType) {
        // skip 3, 7, and 11 because of \n characters
        // index % 15 matches multiples
        println!("{}, {:?}", index % 15, key);
        match index % 15 {
            0 => self.top_left = Some(key),
            1 => self.top_middle = Some(key),
            2 => self.top_right = Some(key),
            4 => self.upper_middle_left = Some(key),
            5 => self.upper_middle_middle = Some(key),
            6 => self.upper_middle_right = Some(key),
            8 => self.lower_middle_left = Some(key),
            9 => self.lower_middle_middle = Some(key),
            10 => self.lower_middle_right = Some(key),
            12 => self.bottom_left = Some(key),
            13 => self.bottom_middle = Some(key),
            14 => self.bottom_right = Some(key),
            _ => eprintln!("Uh oh! Index out of range while trying to construct a number."),
        }
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Number) -> bool {
        self.top_left == other.top_left
            && self.top_middle == other.top_middle
            && self.top_right == other.top_right
            && self.upper_middle_left == other.upper_middle_left
            && self.upper_middle_middle == other.upper_middle_middle
            && self.upper_middle_right == other.upper_middle_right
            && self.lower_middle_left == other.lower_middle_left
            && self.lower_middle_middle == other.lower_middle_middle
            && self.lower_middle_right == other.lower_middle_right
            && self.bottom_left == other.bottom_left
            && self.bottom_middle == other.bottom_middle
            && self.bottom_right == other.bottom_right
    }
}

const ONE: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Space),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Space),
    upper_middle_middle: Some(KeyType::Space),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Space),
    lower_middle_middle: Some(KeyType::Space),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const TWO: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Space),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Pipe),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Space),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const THREE: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Space),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Space),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const FOUR: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Space),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Pipe),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Space),
    lower_middle_middle: Some(KeyType::Space),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const FIVE: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Pipe),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Space),
    lower_middle_left: Some(KeyType::Space),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const SIX: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Pipe),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Space),
    lower_middle_left: Some(KeyType::Pipe),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const SEVEN: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Space),
    upper_middle_middle: Some(KeyType::Space),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Space),
    lower_middle_middle: Some(KeyType::Space),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const EIGHT: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Pipe),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Pipe),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const NINE: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Pipe),
    upper_middle_middle: Some(KeyType::Underscore),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Space),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

const ZERO: Number = Number {
    top_left: Some(KeyType::Space),
    top_middle: Some(KeyType::Underscore),
    top_right: Some(KeyType::Space),
    upper_middle_left: Some(KeyType::Pipe),
    upper_middle_middle: Some(KeyType::Space),
    upper_middle_right: Some(KeyType::Pipe),
    lower_middle_left: Some(KeyType::Pipe),
    lower_middle_middle: Some(KeyType::Underscore),
    lower_middle_right: Some(KeyType::Pipe),
    bottom_left: Some(KeyType::Space),
    bottom_middle: Some(KeyType::Space),
    bottom_right: Some(KeyType::Space),
};

pub fn convert(input: &str) -> Result<String, Error> {
    let (num_rows, num_cols) = verify_rows_and_cols(input)?;
    let num_numbers_vertical = num_rows / 4;
    let num_numbers_horizontal = num_cols / 3;

    read_numbers(input, num_numbers_vertical, num_numbers_horizontal)
}

pub fn verify_rows_and_cols(input: &str) -> Result<(usize, usize), Error> {
    let rows: Vec<&str> = input.lines().collect();

    let num_rows = rows.len();
    if num_rows % 4 != 0 {
        return Err(Error::InvalidRowCount(num_rows));
    }

    let num_cols = rows[0].len();
    if num_cols % 3 != 0 {
        return Err(Error::InvalidColumnCount(num_cols));
    }

    for row in rows {
        if row.len() != num_cols {
            return Err(Error::InvalidColumnCount(row.len()));
        }
    }

    Ok((num_rows, num_cols))
}

pub fn read_numbers(
    input: &str,
    num_numbers_vertical: usize,
    num_numbers_horizontal: usize,
) -> Result<String, Error> {
    let input_vec: Vec<char> = input.chars().collect::<Vec<char>>();
    let _chars_per_num = 12;
    let _total_nums = num_numbers_vertical + num_numbers_horizontal;

    let mut num_string = String::new();
    for i in 0..num_numbers_horizontal {
        let mut number = Number::new();
        // TODO: every 3 chars should skip 3 chars (if there's another number)
        for (index, ch) in input_vec.clone().into_iter().skip(i * 3).enumerate() {
            if ch == '\n' {
                continue;
            } else {
                println!("here is the index: {}, and ch: {}", index, ch);
                let key = KeyType::new(ch);
                number.update(index, key);
            }
        }

        add_num_to_string(&number, &mut num_string);
    }

    // TODO: create string for returning numbers
    // TODO: for each number in numbers vec, match to see if it's an encoded number, else return a
    // ?

    Ok(num_string)
}

pub fn add_num_to_string(num: &Number, num_string: &mut String) {
    println!("{:?} {}", num, num_string);
    if num == &ZERO {
        num_string.push('0')
    } else if num == &ONE {
        num_string.push('1')
    } else if num == &TWO {
        num_string.push('2')
    } else if num == &THREE {
        num_string.push('3')
    } else if num == &FOUR {
        num_string.push('4')
    } else if num == &FIVE {
        num_string.push('5')
    } else if num == &SIX {
        num_string.push('6')
    } else if num == &SEVEN {
        num_string.push('7')
    } else if num == &EIGHT {
        num_string.push('8')
    } else if num == &NINE {
        num_string.push('9')
    } else {
        num_string.push('?')
    }
}
