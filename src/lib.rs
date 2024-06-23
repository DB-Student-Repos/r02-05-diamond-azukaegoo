pub fn get_diamond(c: char) -> Vec<String> {

        // Determine the size based on the given character
        let size = (c as u8 - b'A') as usize;
        let mut diamond = Vec::new();

        // Generate the top half of the diamond (including the middle line)
        for i in 0..=size {
            let mut line = vec![' '; 2 * size + 1];
            let ch = (b'A' + i as u8) as char;
            line[size - i] = ch;
            line[size + i] = ch;
            diamond.push(line.into_iter().collect());
        }

        // Generate the bottom half of the diamond by reversing the top half (excluding the middle line)
        let mut bottom_half = diamond[..size].to_vec();
        bottom_half.reverse();
        diamond.extend(bottom_half);

    diamond
}