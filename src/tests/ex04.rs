/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   ex04.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/07 18:20:58 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/13 11:14:53 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::print_truth_table::print_truth_table;

pub fn ex04() {
	print_truth_table("0");
	print_truth_table("1");
	print_truth_table("A");
	print_truth_table("B");
	print_truth_table("");
	print_truth_table("AB&");
	print_truth_table("AB&C|");
	print_truth_table("AB");
	print_truth_table("GAB&AC&AD^=|>");
}
