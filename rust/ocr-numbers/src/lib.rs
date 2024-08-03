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
        match index {
            0 => self.top_left = Some(key),
            1 => self.top_middle = Some(key),
            2 => self.top_right = Some(key),
            3 => self.upper_middle_left = Some(key),
            4 => self.upper_middle_middle = Some(key),
            5 => self.upper_middle_right = Some(key),
            6 => self.lower_middle_left = Some(key),
            7 => self.lower_middle_middle = Some(key),
            8 => self.lower_middle_right = Some(key),
            9 => self.bottom_left = Some(key),
            10 => self.bottom_middle = Some(key),
            11 => self.bottom_right = Some(key),
            _ => eprintln!("Uh oh! Index out of range while trying to construct a number."),
        }
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
    let input_vec = input.chars().collect::<Vec<char>>();

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
    let chars_per_num = 12;
    let total_nums = num_numbers_vertical + num_numbers_horizontal;

    let mut numbers: Vec<Number> = vec![];
    for (i, _) in (0..num_numbers_horizontal).enumerate() {
        let mut number = Number::new();
        let _ = input_vec
            .chunks(3)
            .skip(i)
            .step_by(2)
            .enumerate()
            .map(|(index, ch)| {
                println!("here");
                println!("here is the index: {}, and ch: {}", index, ch[0]);
                let key = KeyType::new(ch[0]);
                number.update(index, key);
            });
        numbers.push(number);
    }
    println!("{:?}", numbers);

    Ok("done".to_string())
}
