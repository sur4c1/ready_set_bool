/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex08.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 10:14:55 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 11:11:19 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::powerset::powerset;

pub fn ex08 ()
{
	let set = vec![1, 2, 3];
	print!("Powerset of {:?}: ", set);
	println!("{:?}", powerset(set));

	let set = vec![1, 2, 3, 4];
	print!("Powerset of {:?}: ", set);
	println!("{:?}", powerset(set));

	let set = vec![];
	print!("Powerset of {:?}: ", set);
	println!("{:?}", powerset(set));
}
