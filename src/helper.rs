// pad_strings takes vector of rows of columns and pads them
// to evenly display on a terminal. The last column will not
// be padded.
// TODO: Terribly inefficient
// TODO: Probably should return an iterator?
// TODO: There is a redundant copy
pub fn pad_strings(rows: Vec<Vec<String>>) -> Vec<Vec<String>> {
    // maximum length of each column
    let maximums: Vec<usize> = Vec::new();
    let maximums = rows.iter().fold(maximums, |acc, row| {
        // column lengths of each row
        let col_lengths: Vec<usize> = row.iter().map(|x| x.len()).collect();
        let next: Vec<usize> = if acc.len() < col_lengths.len() {
            // grow maximum length vector on first
            // iteration or when rows have a different
            // number of columns
            let other = col_lengths.as_slice().get(acc.len()..).unwrap();
            let mut next = acc.clone();
            next.extend_from_slice(other);
            next
        } else {
            acc
        };
        // check if there is a larger column
        next.iter()
            .enumerate()
            .map(|(i, x)| *std::cmp::max(x, col_lengths.get(i).unwrap()))
            .collect()
    });
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
                        pad_n(col.to_string(), *n - col.len())
                    }
                })
                .collect()
        })
        .collect();
    padded
}

fn pad_n(value: String, n: usize) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < n {
        i += 1;
        out.push(' ')
    }
    out.push_str(value.as_str());
    out
}
