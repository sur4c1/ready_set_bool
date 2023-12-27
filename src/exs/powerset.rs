/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   powerset.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 10:06:41 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 10:16:09 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn	powerset(set: Vec<i32>) -> Vec<Vec<i32>>
{
	let powerset_size: u64 = 1 << set.len(); // 2^set.len() but faster
	let mut powerset = Vec::new();

	if set.len() > 64 {
		eprintln!("powerset: set too big");
		return powerset;
	}

	/*
	 *	NOTE: I use the bijection between the powerset of a set
	 *		and the binary representations of the integers from 0 to 2^n
	 *		where n is the size of the set.
	 */
	for i in 0..powerset_size {
		let mut subset = Vec::new();
		for j in 0..set.len() {
			if i & (1 << j) != 0 // if the jth bit of i is 1
			{
				subset.push(set[j]);
			}
		}
		powerset.push(subset);
	}
	return powerset;
}
