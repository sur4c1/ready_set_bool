/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex00.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 14:43:00 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/12 11:46:51 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::adder::adder;

pub fn ex00() {
	println!("1 + 1 = {} (should be {} with true +)", adder(1, 1), 1 + 1);
	println!("0 + 1 = {} (should be {} with true +)", adder(0, 1), 0 + 1);
	println!("0 + 0 = {} (should be {} with true +)", adder(0, 0), 0 + 0);
	println!(
		"42 + 0 = {} (should be {} with true +)",
		adder(42, 0),
		42 + 0
	);
	println!(
		"0 + 42 = {} (should be {} with true +)",
		adder(0, 42),
		0 + 42
	);
	println!(
		"0 + u32::MAX = {} (should be {} with true +)",
		adder(0, u32::MAX),
		0 + u32::MAX
	);
	println!(
		"1 + u32::MAX = {} (overflow with true +)",
		adder(1, u32::MAX)
	);
}
