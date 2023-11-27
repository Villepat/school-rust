pub use crate::RomanDigit::*;

// Enum representing the Roman digit symbols.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum RomanDigit {
    Nulla, // Represents zero or 'nulla' in Roman
    I,     // Represents 1
    V,     // Represents 5
    X,     // Represents 10
    L,     // Represents 50
    C,     // Represents 100
    D,     // Represents 500
    M,     // Represents 1000
}

// RomanNumber struct, which uses a tuple to group a vector of Roman digits with its
// corresponding numerical value in the form an unsigned 32-bit integer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomanNumber(pub Vec<RomanDigit>, pub u32); // * NEW ADDITION: the tuple now contains the current value

// Struct to group a numerical value with its corresponding Roman digits.
#[derive(Debug, Clone, PartialEq, Eq)]
struct RomanValue {
    value: u32,                 // The numerical value.
    digit1: RomanDigit,         // The main Roman digit.
    digit2: Option<RomanDigit>, // Optional second digit, used in subtractive notation.
}

// Implement the From<u32> trait for RomanDigit.
impl From<u32> for RomanDigit {
    /*
    from() is a function implemented with the From<u32> trait. It takes a an unsigned 32-bit
    integer and returns a RomanDigit. The function uses a match statement to match the given
    number to its Roman digit representation. If the number doesn't match, the function returns
    Nulla.
    */
    fn from(num: u32) -> Self {
        // Match the given number to its Roman digit representation.
        match num {
            1 => RomanDigit::I,
            5 => RomanDigit::V,
            10 => RomanDigit::X,
            50 => RomanDigit::L,
            100 => RomanDigit::C,
            500 => RomanDigit::D,
            1000 => RomanDigit::M,
            _ => RomanDigit::Nulla, // Default case: If the number doesn't match, return Nulla.
        }
    }
}

// Implement the From<u32> trait for RomanNumber.
impl From<u32> for RomanNumber {
    /*
    from() is a function implemented with the From<u32> trait. It takes a an unsigned 32-bit
    integer and returns a RomanNumber. The function uses a vector of RomanDigits to represent
    the Roman number. The function loops through an array of RomanValue structs, which contain
    the numerical value of a Roman numeral and its corresponding Roman digits. The function
    loops through the array, and for each RomanValue, it checks if the number being processed
    is greater than or equal to the RomanValue's value. If it is, it pushes the RomanValue's
    digits to the vector, and subtracts the RomanValue's value from the number being processed.
    */
    fn from(mut num: u32) -> Self {
        // Store the initial value
        let initial_num = num;

        // Initialize an empty vector to hold RomanDigits.
        let mut digits = Vec::new();

        // An array of RomanValue structs representing possible Roman numerals.
        let values = [
            RomanValue {
                value: 1000,
                digit1: RomanDigit::M,
                digit2: None,
            },
            RomanValue {
                value: 900,
                digit1: RomanDigit::C,
                digit2: Some(RomanDigit::M),
            },
            RomanValue {
                value: 500,
                digit1: RomanDigit::D,
                digit2: None,
            },
            RomanValue {
                value: 400,
                digit1: RomanDigit::C,
                digit2: Some(RomanDigit::D),
            },
            RomanValue {
                value: 100,
                digit1: RomanDigit::C,
                digit2: None,
            },
            RomanValue {
                value: 90,
                digit1: RomanDigit::X,
                digit2: Some(RomanDigit::C),
            },
            RomanValue {
                value: 50,
                digit1: RomanDigit::L,
                digit2: None,
            },
            RomanValue {
                value: 40,
                digit1: RomanDigit::X,
                digit2: Some(RomanDigit::L),
            },
            RomanValue {
                value: 10,
                digit1: RomanDigit::X,
                digit2: None,
            },
            RomanValue {
                value: 9,
                digit1: RomanDigit::I,
                digit2: Some(RomanDigit::X),
            },
            RomanValue {
                value: 5,
                digit1: RomanDigit::V,
                digit2: None,
            },
            RomanValue {
                value: 4,
                digit1: RomanDigit::I,
                digit2: Some(RomanDigit::V),
            },
            RomanValue {
                value: 1,
                digit1: RomanDigit::I,
                digit2: None,
            },
        ];

        // Loop through each RomanValue in the values array.
        for value in values.iter() {
            // While the part of the number being processed is greater than or equal to
            // the current RomanValue's value, push the RomanValue's digits to the vector.
            while num >= value.value {
                // Push the first Roman digit.
                digits.push(value.digit1);

                // If there's a second Roman digit (for subtractive notation), push it too.
                if let Some(digit2) = value.digit2 {
                    digits.push(digit2);
                }

                // Subtract the value of the RomanValue from the number being processed.
                num -= value.value;
            }
        }

        // Return a RomanNumber struct containing the vector of RomanDigits and the current value.
        RomanNumber(digits, initial_num) // * NEW ADDITION: the current value is returned with the new RomanNumber definition
    }
}

/*
* NEW ADDITION for this task
*/
impl Iterator for RomanNumber {
    // The associated type Item will be RomanNumber
    // ! Item is the type of the elements being iterated over and is required by the Iterator trait.
    type Item = RomanNumber;

    /*
    The next method is the core of an Iterator. It returns an Option<Self::Item>
    In this case, it will return an Option<RomanNumber>. If the Iterator has more
    items, return Some(item), otherwise return None.
    */
    fn next(&mut self) -> Option<Self::Item> {
        // Increment the current value by 1
        self.1 += 1;

        // Convert the new current value to its Roman digit representation
        let new_digits = RomanNumber::from(self.1).0;
        // ... and update self.0 (the RomanNumber's digits) with the new digits
        self.0 = new_digits.clone();

        // Return a new RomanNumber instance with the updated fields
        Some(RomanNumber(new_digits, self.1))
    }
}
