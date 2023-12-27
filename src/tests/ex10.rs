/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex10.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 12:05:55 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 12:11:24 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::map::map;

pub fn ex10 ()
{
	let x = 0;
	let y = 0;
	let res = map(x, y);
	println!("map({}, {}) = {}", x, y, res);

	let x = 0xFFFF;
	let y = 0xFFFF;
	let res = map(x, y);
	println!("map({}, {}) = {}", x, y, res);

	let x = 42;
	let y = 0;
	let res = map(x, y);
	println!("map({}, {}) = {}", x, y, res);

	let x = 0;
	let y = 42;
	let res = map(x, y);
	println!("map({}, {}) = {}", x, y, res);
}
