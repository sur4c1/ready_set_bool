/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex02.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 15:20:47 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/05 15:29:36 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::gray_code::gray_code;

pub fn ex02() {
	println!("grey_code(0) = {} (should be 0)", gray_code(0));
	println!("grey_code(1) = {} (should be 1)", gray_code(1));
	println!("grey_code(2) = {} (should be 3)", gray_code(2));
	println!("grey_code(3) = {} (should be 2)", gray_code(3));
	println!("grey_code(4) = {} (should be 6)", gray_code(4));
	println!("grey_code(5) = {} (should be 7)", gray_code(5));
	println!("grey_code(6) = {} (should be 5)", gray_code(6));
	println!("grey_code(7) = {} (should be 4)", gray_code(7));
	println!("grey_code(8) = {} (should be 12)", gray_code(8));
	println!("grey_code(42) = {} (should be 63)", gray_code(42));
}
