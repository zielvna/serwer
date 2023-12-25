use crate::SerwerError;

pub fn decode(string: &str) -> Result<String, SerwerError> {
    let mut result = String::new();
    let mut chars = string.chars();

    while let Some(char) = chars.next() {
        if char == '%' {
            let mut hex = String::new();

            hex.push(
                chars
                    .next()
                    .ok_or(SerwerError::DecodeError(String::from(string)))?,
            );
            hex.push(
                chars
                    .next()
                    .ok_or(SerwerError::DecodeError(String::from(string)))?,
            );

            let decoded = u8::from_str_radix(&hex, 16)?;

            result.push(decoded as char);
        } else {
            result.push(char);
        }
    }

    let result = result.replace("+", " ");

    Ok(result)
}
