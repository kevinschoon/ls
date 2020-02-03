// pad_strings takes vector of rows of columns and pads them
// to evenly display on a terminal. The last column will not
// be padded.
pub fn pad_strings(rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    // maximum length of each column of each row
    let mut maximums: Vec<usize> = Vec::new();
    for row in rows.iter() {
        // cell lengths of each column in this row
        let col_lengths: Vec<usize> = row.iter().map(|x| x.len()).collect();
        // grow the maximums column vector as needed
        if maximums.len() < row.len() {
            let adjusted = col_lengths.get(maximums.len()..).unwrap();
            maximums.extend_from_slice(adjusted);
        }
        // check for larger cells
        maximums = maximums
            .iter()
            .enumerate()
            .map(|(i, len)| *std::cmp::max(len, col_lengths.get(i).unwrap()))
            .collect()
    }
    // pad each column with the maximum column length
    let padded: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            let row_len = row.len();
            row.iter()
                .enumerate()
                .map(|(i, col)| {
                    if i == row_len - 1 {
                        String::from(col)
                    } else {
                        let n = maximums.get(i).unwrap();
                        let mut padded_cell = " ".repeat(*n - col.len());
                        padded_cell.push_str(col.as_str());
                        padded_cell
                    }
                })
                .collect()
        })
        .collect();
    padded
}
