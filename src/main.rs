// use core::fmt::{self, Display, Formatter};
use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter line:");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let line = Line::new(input.trim().to_string());

    match line {
        Ok(val) => {
            println!("{:?}", val);
            println!("{:?}", val.decimal);
            println!("{:?}", val.american);
        }
        Err(error) => println!("{}", error),
    }
}

#[derive(Debug)]
struct Line {
    american: Option<AmericanLine>,
    decimal: Option<DecimalLine>,
}

impl Line {
    fn new(input: String) -> Result<Self, String> {
        // parse number
        let number: Result<f32, _> = input.parse();

        // handle errors
        match number {
            Ok(parsed_number) => {
                // determine format
                if parsed_number < -100.0 || parsed_number > 100.0 {
                    // format is American, return line
                    let line: AmericanLine = AmericanLine {
                        line: parsed_number.round() as i32,
                    };
                    return Ok(Line {
                        american: Some(line),
                        // convert american to decimal
                        decimal: Some(DecimalLine {
                            line: line.to_decimal(),
                        }),
                    });
                } else if parsed_number >= 1.0 {
                    // format is Decimal, return line
                    let line: DecimalLine = DecimalLine {
                        line: parsed_number,
                    };
                    return Ok(Line {
                        // convert decimal to American
                        american: Some(AmericanLine {
                            line: line.to_american(),
                        }),
                        decimal: Some(line),
                    });
                } else {
                    return Err("Failed to determine input line format.".to_string());
                }
            }
            Err(_) => return Err("Failed to parse number.".to_string()),
        }
    }
}

trait LineType {
    fn implied_probability(&self) -> f32;
    fn to_decimal(&self) -> f32;
    fn to_american(&self) -> i32;
}

#[derive(Copy, Clone, Debug)]
struct AmericanLine {
    line: i32,
}

#[derive(Copy, Clone, Debug)]
struct DecimalLine {
    line: f32,
}

impl LineType for AmericanLine {
    fn implied_probability(&self) -> f32 {
        if self.line >= 100 {
            return 100.0 / (100.0 + (self.line as f32));
        } else {
            return (self.line.abs() as f32) / (100.0 + (self.line.abs() as f32));
        }
    }

    fn to_american(&self) -> i32 {
        return self.line;
    }

    fn to_decimal(&self) -> f32 {
        return 1.0 / self.implied_probability();
    }
}

impl LineType for DecimalLine {
    fn implied_probability(&self) -> f32 {
        return 1.0 / self.line;
    }

    fn to_american(&self) -> i32 {
        let implied: f32 = self.implied_probability();
        if implied > 0.5 {
            return ((100.0 * implied) / (implied - 1.0)) as i32;
        } else {
            return ((100.0 / implied) - 100.0) as i32;
        }
    }

    fn to_decimal(&self) -> f32 {
        return self.line;
    }
}
