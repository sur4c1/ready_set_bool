/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 11:06:13 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/05 11:17:21 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub mod	adder;

fn	main()
{
	println!("1 + 2 = {}", adder::adder(1, 2));
	println!("2 + 1 = {}", adder::adder(2, 1));
	println!("0 + INT_MAX = {}", adder::adder(0, u32::MAX));
	println!("1 + INT_MAX = {} (should overflow)", adder::adder(1, u32::MAX));
}
