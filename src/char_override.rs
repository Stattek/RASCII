use unicode_segmentation::UnicodeSegmentation;

/// Converts a string into a vector of strings, with a character each for every string.
/// This is a workaround to deal with the weirdness of UTF-8, since we cannot index into a string
/// and just grab a character (we could get some garbage byte that doesn't map to a character).
///
/// # Params
/// - `the_str` - The string to convert into a string vector.
///
/// # Returns
/// - A `Vec<String>`, with each element of the vec containing one character each.
pub fn convert_string_to_str_vec(the_str: String) -> Vec<String> {
    let graphemes = the_str.graphemes(true);
    let mut output_vec = Vec::new();
    for cur_char in graphemes {
        output_vec.push(String::from(cur_char));
    }

    output_vec
}
