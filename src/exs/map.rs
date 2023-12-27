/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   map.rs                                             :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 11:42:01 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 12:09:23 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn map(x: u16, y: u16) ->f64
{
	let x = x as f64;
	let y = y as f64;

	return x / ((1 << 16) as f64) + y / (((1 as u64) << 32) as f64);
}

/*
 *		Let I = [[0, 2^16 - 1]] x [[0, 2^16 - 1]]
 *		Let A = {x / 2^16 + y / 2^32 | (x, y) in I } subset of [0, 1]
 *		f:	I		->	A
 *		f:	(x, y)	->	x / 2^16 + y / 2^32
 *
 * Let's show that f is a bijection:
 * 	- f is injective:
 * 		Let (x1, y1) and (x2, y2) two elements of A
 * 			such that f(x1, y1) = f(x2, y2)
 * 		Therefore:
 *			x1 / 2^16 + y1 / 2^32 = x2 / 2^16 + y2 / 2^32
 *			x1 / 2^16 - x2 / 2^16 = y2 / 2^32 - y1 / 2^32
 *			(x1 - x2) / 2^16 = (y2 - y1) / 2^32
 *			(x1 - x2) / 2^16 + 0 / 2^32 = 0 / 2^16 + (y2 - y1) / 2^32
 *		Since 0 <= y1, y2 < 2^16 => y2 - y1 < 2^16 => (y2 - y1) / 2^32 < 2^16
 *		Therefore, since x1 and x2 are integers, we can infer that
 *		(x1 - x2) / 2^16 = 0	and		(y2 - y1) / 2^32 = 0
 *		Therefore, x1 = x2 and y1 = y2
 *		:)
 *	- f is surjective:
 *		Let a an element of A
 *		By definition of A, a = x / 2^16 + y / 2^32 for some (x, y) in I
 *		Therefore, f(x, y) = a
 *		:)
 *	:)
 */
