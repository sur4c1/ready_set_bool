/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex01.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 15:11:43 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/05 15:15:02 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::multiplier::multiplier;

pub fn ex01() {
	println!(
		"1 * 1 = {} (should be {} with true *)",
		multiplier(1, 1),
		1 * 1
	);
	println!(
		"1 * 6 = {} (should be {} with true *)",
		multiplier(1, 6),
		1 * 6
	);
	println!(
		"3 * 14 = {} (should be {} with true *)",
		multiplier(3, 14),
		3 * 14
	);
	println!(
		"14 * 3 = {} (should be {} with true *)",
		multiplier(14, 3),
		14 * 3
	);
	println!(
		"42 * 12 = {} (should be {} with true *)",
		multiplier(42, 12),
		42 * 12
	);
	println!(
		"0 * 42 = {} (should be {} with true *)",
		multiplier(0, 42),
		0 * 42
	);
	println!(
		"1 * u32::MAX = {} (should be {} with true *)",
		multiplier(1, u32::MAX),
		1 * u32::MAX
	);
	println!(
		"2 * (u32::MAX - 1) = {} (overflow with true *)",
		multiplier(2, u32::MAX - 1)
	);
}
