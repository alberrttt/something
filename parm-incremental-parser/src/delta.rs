use parm_ast::prelude::*;
fn display_matrix(matrix: &[Vec<usize>]) {
    for row in matrix {
        for col in row {
            print!("{} ", col);
        }
        println!();
    }
}
fn display_myer_diff(matrix: &[Vec<usize>], tokens1: &[Token], tokens2: &[Token]) {
    for (row_idx, row) in matrix.iter().enumerate() {
        if row_idx != 0 {
            continue;
        };
        print!("    ");
        for (col_idx, col) in row.iter().enumerate() {
            if col_idx == 0 {
                continue;
            }
            print!("{} ", tokens1[col_idx - 1].lexeme());
        }
    }
    println!();
    for (row_idx, row) in matrix.iter().enumerate() {
        if (row_idx > 0) {
            print!("{} ", tokens2[row_idx - 1].lexeme());
        } else {
            print!("  ")
        }
        for (col_idx, col) in row.iter().enumerate() {
            print!("{} ", col);
        }
        println!();
    }
}
fn diff_tokens(tokens1: &[Token], tokens2: &[Token]) {
    let n = tokens1.len();
    let m = tokens2.len();

    let mut matrix = vec![vec![0; m + 1]; n + 1];
    for (index, value) in matrix.iter().enumerate() {
        if (index == 0) {
            continue;
        }
        let token_index = index - 1;
        println!("{}", tokens1[token_index].lexeme())
    }
    // display_myer_diff(&matrix, tokens1, tokens2)
}

#[test]
fn test() {
    let code1 = "a b c d";
    let code2 = "a b c d";

    let tokens1 = Lexer::from(code1).lex();
    let tokens2 = Lexer::from(code2).lex();

    let edits = diff_tokens(&tokens1, &tokens2);
}
