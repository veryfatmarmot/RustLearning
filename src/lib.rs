use std::error::Error;

/// Holds parameters of a rectangle in integer representation.
///
/// # Examples
///
/// ```
/// use sergii_bondar_dummy_crate::Rectangle;
///
/// let rec = Rectangle { width: 10, height: 1 };
/// assert_eq!(10, rec.width);
/// assert_eq!(1, rec.height);
/// ```
#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut list = [
        Rectangle {
            width: 10,
            height: 1,
        },
        Rectangle {
            width: 3,
            height: 5,
        },
        Rectangle {
            width: 7,
            height: 12,
        },
    ];

    let mut sort_operations = vec![];
    let value = String::from("closure called");

    list.sort_by_key(|r| {
        sort_operations.push(value.clone());
        r.width
    });
    println!("{list:#?}");

    Ok(())
}

// ==================================================================================================================
/*
#[cfg(test)]
mod tests {
    use super::*;

}*/
