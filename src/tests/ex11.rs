/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex11.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 16:15:13 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 16:20:15 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::reverse_map::reverse_map;
use crate::exs::map::map;

pub fn ex11()
{
	let mut n = 0.0;
	while n < 1.0 {
		let (x, y) = reverse_map(n);
		println!("{} -> ({}, {})", n, x, y);
		n += 0.21;
	}

	for x in 0..7 {
		for y in 0..8 {
			let x = x * 8;
			let y = y * 7;
			let n = map(x, y);
			let (x_, y_) = reverse_map(n);
				println!("({}, {}) -> {} -> ({}, {})", x, y, n, x_, y_);
		}
	}
}
