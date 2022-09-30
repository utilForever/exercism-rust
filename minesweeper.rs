pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row_idx, row_content)| {
            row_content
                .chars()
                .enumerate()
                .map(|(col_idx, col_content)| {
                    if col_content == '*' {
                        '*'
                    } else {
                        let mut count = 0;

                        for i in row_idx.saturating_sub(1)..=row_idx + 1 {
                            for j in col_idx.saturating_sub(1)..=col_idx + 1 {
                                if i < minefield.len()
                                    && j < row_content.len()
                                    && minefield[i].chars().nth(j) == Some('*')
                                {
                                    count += 1;
                                }
                            }
                        }

                        if count == 0 {
                            ' '
                        } else {
                            (count as u8 + b'0') as char
                        }
                    }
                })
                .collect()
        })
        .collect()
}
