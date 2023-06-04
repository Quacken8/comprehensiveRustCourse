fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {

    let mut transposed = [[0; 3]; 3]; // start with allocating an empty array

    for row in 0..matrix.len() { // now just transpose using loop
        for col in 0..matrix[row].len() {
            transposed[col][row] = matrix[row][col];
        }
    }

    // I wasn't asked to return a mutable array
    let transposed = transposed;

    return transposed;
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for row in matrix {
        println!("{:?}", row);
    }
}

fn main() {
    let matrix = [
        [101, 102, 103], //
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
