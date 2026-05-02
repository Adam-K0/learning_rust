pub fn transpose(mat: &Vec<Vec<usize>> ) -> Vec<Vec<usize>> {
	let mut transpose = vec![vec![0; mat.len()]; mat[0].len()] ;
	// uncomment for debugging
	// println!("mat's dims = { } by { } ", mat.len(), mat[0].len() );
	// println!("transpose's dims = { } rows by { } cols", transpose.len(), transpose[0].len() );

	for row in 0..mat.len() {
		for col in 0..mat[0].len(){
			transpose[col][row] = mat[row][col];
		}
	}
	
	transpose	
}
pub fn scale_row(mat: & mut Vec<Vec<isize>>, row: usize, scale: isize) -> Vec<Vec<isize>>{
	for entry in 0..mat[row].len() {
		mat[row][entry] = mat[row][entry] * scale;
	}
	mat.to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;
	// func names are {num rows}_by_{num cols}
    #[test]
    fn transpose_3x3() {
		let mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
        let result = transpose(&mat);
		let correct = vec![vec![1,4,7], vec![2,5,8], vec![3,6,9]];
        assert_eq!(result, correct);
    }
	#[test]
	fn transpose_3x4(){
		let mat = vec![vec![1,2,3,4], vec![5,6,7,8], vec![9,10,11,12]];
		let result = transpose(&mat);
		let correct = vec![vec![1,5,9], vec![2,6,10], vec![3,7,11], vec![4,8,12]];
		assert_eq!(result, correct);
	}
	#[test]
	fn scale_2x2(){
		let mut mat = vec![vec![44,100], vec![0, -44]];
		let result = scale_row(& mut mat, 1, 2);
		let correct = vec![vec![44,100], vec![0, -88]];
		let incorrect = vec![vec![44,100], vec![0, 88]];
		assert_eq!(result, correct);
		assert_ne!(result, incorrect);
	}
	
}
