use crate::solutions::day03;

fn is_newline(s: &String, idx: usize) -> bool {
    s.as_bytes()[idx] == '\n' as u8
}

fn is_in_bounds(s: &String, idx: usize) -> bool {
    idx < s.len()
}

pub fn find_line_len(input: &str) -> usize {
    input.find('\n').unwrap() + 1
}

fn row(input: &String, index: usize) {}

/// Returns a vector of valid indexes over the string. Valid indexes are non newline characters within the string
pub fn adjacent_indexes(input: &String, index: usize, include_diagonals: bool) -> Vec<usize> {
    let mut indexes = vec![];
    let line_len = find_line_len(input);
    let index_less_one = if index.checked_sub(1).is_some_and(|i| !is_newline(input, i)) { Some(index - 1) } else { None };
    let index_plus_one = if is_newline(input, index + 1) { None } else { Some(index + 1) };

    if let Some(i) = index.checked_sub(line_len) { indexes.push(i) }
    if is_in_bounds(input, index + line_len) { indexes.push(index + line_len) }

    if let Some(i) = index_less_one {
        indexes.push(i);
        if include_diagonals {
            if let Some(i_less_row) = i.checked_sub(line_len) { indexes.push(i_less_row) }
            if is_in_bounds(input, i + line_len) { indexes.push(i + line_len) }
        }
    }
    if let Some(i) = index_plus_one {
        indexes.push(i);
        if include_diagonals {
            if let Some(i_less_row) = i.checked_sub(line_len) { indexes.push(i_less_row) }
            if is_in_bounds(input, i + line_len) { indexes.push(i + line_len) }
        }
    }

    indexes.sort();
    indexes
}

mod test {
    use crate::solutions::string_grid::adjacent_indexes;

    #[test]
    fn simple_grid() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 5, true), vec![0, 1, 2, 4, 6, 8, 9, 10])
    }

    #[test]
    fn simple_grid_h_edge() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 1, true), vec![0, 2, 4, 5, 6])
    }

    #[test]
    fn simple_grid_corner() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 0, true), vec![1, 4, 5])
    }

    #[test]
    fn simple_grid_v_edge() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 4, true), vec![0, 1, 5, 8, 9])
    }

    #[test]
    fn simple_grid_alt_corner() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 10, true), vec![5, 6, 9])
    }


    #[test]
    fn simple_grid_no_diag() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 5, false), vec![1, 4, 6, 9])
    }

    #[test]
    fn simple_grid_h_edge_no_diag() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 1, false), vec![0, 2, 5])
    }

    #[test]
    fn simple_grid_corner_no_diag() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 0, false), vec![1, 4])
    }

    #[test]
    fn simple_grid_v_edge_no_diag() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 4, false), vec![0, 5, 8])
    }

    #[test]
    fn simple_grid_alt_corner_no_diag() {
        let s =
            "123\n\
             456\n\
             789\n".to_string();
        assert_eq!(adjacent_indexes(&s, 10, false), vec![6, 9])
    }
}
