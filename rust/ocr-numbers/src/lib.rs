// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub enum KeyType {
    Pipe,
    Underscore,
    Space,
}

pub struct Number {
    top_left: KeyType,
    top_middle: KeyType,
    top_right: KeyType,
    upper_middle_left: KeyType,
    upper_middle_middle: KeyType,
    upper_middle_right: KeyType,
    lower_middle_left: KeyType,
    lower_middle_middle: KeyType,
    lower_middle_right: KeyType,
    bottom_left: KeyType,
    bottom_middle: KeyType,
    bottom_right: KeyType,
}

const ONE: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Space,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Space,
    upper_middle_middle: KeyType::Space,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Space,
    lower_middle_middle: KeyType::Space,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const TWO: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Space,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Pipe,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Space,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const THREE: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Space,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Space,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const FOUR: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Space,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Pipe,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Space,
    lower_middle_middle: KeyType::Space,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const FIVE: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Pipe,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Space,
    lower_middle_left: KeyType::Space,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const SIX: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Pipe,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Space,
    lower_middle_left: KeyType::Pipe,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const SEVEN: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Space,
    upper_middle_middle: KeyType::Space,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Space,
    lower_middle_middle: KeyType::Space,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const EIGHT: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Pipe,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Pipe,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const NINE: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Pipe,
    upper_middle_middle: KeyType::Underscore,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Space,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

const ZERO: Number = Number {
    top_left: KeyType::Space,
    top_middle: KeyType::Underscore,
    top_right: KeyType::Space,
    upper_middle_left: KeyType::Pipe,
    upper_middle_middle: KeyType::Space,
    upper_middle_right: KeyType::Pipe,
    lower_middle_left: KeyType::Pipe,
    lower_middle_middle: KeyType::Underscore,
    lower_middle_right: KeyType::Pipe,
    bottom_left: KeyType::Space,
    bottom_middle: KeyType::Space,
    bottom_right: KeyType::Space,
};

pub fn convert(input: &str) -> Result<String, Error> {
    // TODO: determine if cols and rows are correct dimensions (3cols, 4rows)
    let input_vec = input.chars().collect::<Vec<char>>();
    println!("{:?}", input_vec);

    verify_rows_and_cols(input)?;

    // TODO: iterate over str type; after every 3 chars, a new row starts
    // TODO: add each char to the Number struct. if invalid, put a ?
    // TODO: Encode each Number
    //
    //

    Ok("String".to_string())
}

pub fn verify_rows_and_cols(input: &str) -> Result<(), Error> {
    let rows: Vec<&str> = input.lines().collect();

    if rows.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(rows.len()));
    }

    let num_columns = rows[0].len();
    if num_columns % 3 != 0 {
        return Err(Error::InvalidColumnCount(num_columns));
    }

    for row in rows {
        if row.len() != num_columns {
            return Err(Error::InvalidColumnCount(row.len()));
        }
    }

    Ok(())
}
