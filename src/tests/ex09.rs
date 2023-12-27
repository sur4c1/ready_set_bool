/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex09.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 11:29:27 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 11:40:21 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::eval_set::eval_set;

pub fn ex09 ()
{
	let sets = vec![vec![0, 1, 2], vec![0, 3, 4]];
	print!("Result of AB& with {:?}", sets);
	let result = eval_set("AB&", sets);
	println!(": {:?}", result);

	let sets = vec![vec![0, 1, 2], vec![3, 4, 5]];
	print!("Result of AB| with {:?}", sets);
	let result = eval_set("AB|", sets);
	println!(": {:?}", result);

	let sets = vec![vec![0, 1 , 2]];
	print!("Result of A! with {:?}", sets);
	let result = eval_set("A!", sets);
	println!(": {:?}", result);

	let sets = vec![vec![0, 1, 2], vec![0, 3, 4], vec![-1]];
	print!("Result of AB= with {:?}", sets);
	let result = eval_set("AB=", sets);
	println!(": {:?}", result);

	let sets = vec![vec![0, 1, 2], vec![0, 3, 4], vec![-1]];
	print!("Result of AB> with {:?}", sets);
	let result = eval_set("AB>", sets);
	println!(": {:?}", result);

}
