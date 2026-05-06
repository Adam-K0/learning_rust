pub fn transpose(mat: &Vec<Vec<isize>> ) -> Vec<Vec<isize>> {
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
mod elementary_row_operations{

	fn scale_entries(entries: &mut [isize], scale: isize) {
		for entry in entries {
			*entry *= scale;
		}
	}

	pub fn scale_mat(mat: &mut [Vec<isize>], row: usize, scale: isize) -> Vec<Vec<isize>>{
		scale_entries(&mut mat[row], scale);
		mat.to_vec()
	}

	pub fn scale_vec(vec: &mut [isize], scale: isize) -> Vec<isize> {
		scale_entries(vec, scale);
		vec.to_vec()
	}

	// combines matrix's row 1 and row 2 replacing row 2
	pub fn combine(mat: &mut Vec<Vec<isize>>, row1_index: usize, row2_index: usize) -> Vec<Vec<isize>> {
		for entry in 0..mat[row2_index].len() {
			mat[row2_index][entry] += mat[row1_index][entry];
		}
		mat.to_vec()
	}

	pub fn swap(mat: &mut Vec<Vec<isize>>, row1_index: usize, row2_index: usize) -> Vec<Vec<isize>> {
		let plcholder: Vec<isize> = mat[row2_index].clone();
		mat[row2_index] = mat[row1_index].clone();
		mat[row1_index] = plcholder;

		mat.to_vec()
	}
}

mod vector{
	pub fn dot_prod(vec1: &Vec<isize>, vec2: &Vec<isize>) -> Option< isize >{
		if vec1.len() != vec2.len(){
			return None
		}
		let mut result: isize = 0;
		for i in 0..vec1.len(){
			result += vec1[i] * vec2[i]	;
		}
		Some(result)
	}

	pub fn magnitude(vec: &Vec<isize>) -> isize{
		// unwrap is chill here cuz the arguements guaranteed to be same length
		return dot_prod(&vec, &vec).unwrap().isqrt()
	}

	pub fn normalize(vec: & mut Vec<isize>) {
		let mag = magnitude(&vec);
		for entry in vec {
			*entry *= (1/mag.isqrt());
		}
	}
}


pub fn mat_mul(mat1: &Vec<Vec<isize>>, mat2: &Vec<Vec<isize>>) -> Option< Vec<Vec<isize>> > {
	if mat1[0].len() != mat2.len(){
		return None
	}
	let mut result = vec![vec![0; mat2[0].len()]; mat1.len()];

	for row in 0..mat1.len() {
		for col in 0..mat2[0].len(){
			let mut sum  = 0;
			for entry in 0..mat2.len(){
				sum += mat1[row][entry] * mat2[entry][col];
			}
			result[row][col] = sum; 
		}
	}
	Some(result.to_vec())
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
		let result = elementary_row_operations::scale_mat(& mut mat, 1, 2);
		let correct = vec![vec![44,100], vec![0, -88]];
		let incorrect = vec![vec![44,100], vec![0, 88]];

		assert_eq!(result, correct);
		assert_ne!(result, incorrect);
	}

	#[test]
	#[should_panic]
	fn scale_2x2_bad_row_index(){
		let mut mat = vec![vec![44,100], vec![0, -44]];
		let result = elementary_row_operations::scale_mat(& mut mat, 3, 2);
		let correct = vec![vec![44,100], vec![0, -88]];
		let incorrect = vec![vec![44,100], vec![0, 88]];

		assert_eq!(result, correct);
		assert_ne!(result, incorrect);
	}

	#[test]
	fn scale_vec_3x1(){
		let mut vec = vec![44, 0, -44];
		let result = elementary_row_operations::scale_vec(&mut vec, 2);
		let correct = vec![88, 0, -88];

		assert_eq!(result, correct);
	}

	#[test]
	fn combine_rows(){
		let mut mat = vec![vec![44,100], vec![0, -44]];
		let result = elementary_row_operations::combine(& mut mat, 0, 1);
		let correct = vec![vec![44,100], vec![44, 56]];

		assert_eq!(result, correct);
	}

	#[test]
	fn swap_rows(){
		let mut mat = vec![vec![44,100], vec![0, -44]];
		let result = elementary_row_operations::swap(& mut mat, 0, 1);
		let correct = vec![vec![0,-44], vec![44,100]];

		assert_eq!(result, correct);
	}

	#[test]	
	fn mat_mul_identity(){
		let mat1 = vec![vec![1,0], vec![0, 1]];
		let mat2 = vec![vec![44,100], vec![0, -44]];
		let result = mat_mul(&mat1, &mat2).unwrap();
		let correct = mat2; 
		
		assert_eq!(result, correct);
	}

	#[test]
	fn mat_mul_shared_dim(){
		let mat1 = vec![vec![1,2], vec![-2,1]];
		let mat2 = vec![vec![3,3], vec![1,1]];
		let result = mat_mul(&mat1, &mat2).unwrap();
		let correct = vec![vec![5,5], vec![-5,-5]]; 

		assert_eq!(result, correct);
	}
	
	#[test]
	fn mat_mul_different_dim(){
		let mat1 = vec![vec![1,2], vec![-2,1]];
		let mat2 = vec![vec![1; 4]; 2];
		let result = mat_mul(&mat1, &mat2).unwrap();
		let correct = vec![vec![3; 4], vec![-1; 4]] ;

		assert_eq!(result, correct);
	}

	#[test]
	#[should_panic]
	fn mat_mul_wrong_dim(){
		let mat1 = vec![vec![1,2,3], vec![11,12,13]];
		let mat2 = vec![vec![1,2], vec![11, 12]];
		let result = mat_mul(&mat1, &mat2);
		result.unwrap();
	}

	#[test]
	fn dot_prod_3x1(){
		let vec1 = vec![1,2,3];
		let vec2 = vec![101,202,303];
		let res = vector::dot_prod(&vec1, &vec2).unwrap();
		let correct = 101+404+909;

		assert_eq!(res, correct);
	}

	#[test]
	#[should_panic]
	fn dot_prod_mismatch_lens(){
		let vec1 = vec![1,2,3,4];
		let vec2 = vec![1,2,3,4,5];
		let res = vector::dot_prod(&vec1, &vec2);
		res.unwrap();
	}

	#[test]
	fn magnitude_3x1(){
		let vec = vec![4,-5,-4];
		let res = vector::magnitude(&vec);
		let correct = 57_isize.isqrt();
		assert_eq!(res, correct);
	}

	
	#[test]
	fn normalize_10x1(){
		let mut vec = vec![2,2,2,3,3,3,7,7,7,10];
		vector::normalize(&mut vec);
		let mut vec2 = vec![2,2,2,3,3,3,7,7,7,10];
		let correct = elementary_row_operations::scale_vec(&mut vec2, 1/(286_isize.isqrt()));
		assert_eq!(vec, correct);
	}
	
}
