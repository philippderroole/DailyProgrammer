use std::time::Instant;

fn main() {
    let matrix = vec![
        vec![ 123456789,  752880530, 826085747,  576968456, 721429729],
        vec![ 173957326, 1031077599, 407299684,   67656429,  96549194],
        vec![1048156299,  663035648, 604085049, 1017819398, 325233271],
        vec![ 942914780,  664359365, 770319362,   52838563, 720059384],
        vec![ 472459921,  662187582, 163882767,  987977812, 394465693],
    ];
    benchmark(matrix, 1099762961);
}

fn benchmark(matrix: Vec<Vec<u32>>, correct_result: u32) {
    let start = Instant::now();
    let result = smallest_sum_matrix(matrix);
    let duration = start.elapsed();
    println!("Time elapsed in 5by5 is: {:?}", duration);
    println!("Result ({}) = {}", correct_result, result);
}

fn smallest_sum_matrix<'a>(matrix: Vec<Vec<u32>>) -> u64 {
    let size = matrix.len();
    let mut min_sum = u64::MAX;
    let selections = generate_selections(size);

    for selection in selections {
        let vector = vector_from_matrix(&matrix, &selection);
        let vector = vector.iter().map(|&x| x as u64).collect();
        let sum = sum_selection(&vector);

        if sum < min_sum {
            min_sum = sum;
        }
    }
    
    min_sum
}

fn vector_from_matrix(matrix: &Vec<Vec<u32>>, selection: &Vec<usize>) -> Vec<u32>{
    let mut vector = Vec::new();
    
    for i in 0..selection.len() {
        vector.push(matrix[selection[i]][i])
    }

    vector
}

fn generate_selections(size: usize) -> Vec<Vec<usize>> {
    let mut options = Vec::new();

    for index in 0..size {
        options.push(index);
    }

    generate_selections_recursive(&options)
}

fn generate_selections_recursive(options: &Vec<usize>) -> Vec<Vec<usize>> {
    let mut ret: Vec<Vec<usize>> = Vec::new();

    if options.len() == 1 {
        return vec![vec![options[0]]];
    }

    for index in 0..options.len() {
        // remove option from new_options
        let mut new_options = options.clone();
        let option = new_options.swap_remove(index);

        // insert option in front of results and add it to ret
        let results = generate_selections_recursive(&new_options);
        for mut result in results {
            result.insert(0, option);
            ret.push(result);
        }
    }

    ret
}

fn sum_selection(selection: &Vec<u64>) -> u64 {
    selection.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::smallest_sum_matrix;

    #[test]
    fn matrix_simple() {
        let matrix = vec![
            vec![1,10],
            vec![10,1]
        ];

        let result = smallest_sum_matrix(matrix);
        assert_eq!(result, 2);
    }
    
    #[test]
    fn matrix_5by5() {
        let matrix = vec![
            vec![ 123456789,  752880530, 826085747,  576968456, 721429729],
            vec![ 173957326, 1031077599, 407299684,   67656429,  96549194],
            vec![1048156299,  663035648, 604085049, 1017819398, 325233271],
            vec![ 942914780,  664359365, 770319362,   52838563, 720059384],
            vec![ 472459921,  662187582, 163882767,  987977812, 394465693],
        ];

        let result = smallest_sum_matrix(matrix);
        assert_eq!(result, 1099762961);
    }
}