/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   multiplier.rs                                      :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 15:08:50 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 16:26:05 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::adder::adder;

pub fn multiplier(a: u32, b: u32) -> u32 {
	let mut a = a;
	let mut b = b;
	let mut res = 0;

	while b != 0 {
		if b & 1 != 0 {
			res = adder(res, a);
		}
		b >>= 1;
		a <<= 1;
	}
	return res;
}
