/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   print_truth_table.rs                               :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 16:33:52 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/05 18:08:01 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use	crate::exs::eval_formula::eval_formula_opt;
use	std::collections::HashMap;

pub fn	print_truth_table(formula: &str)
{
	let mut	i: u32 = 0;
	let mut nb_var: u32 = 0;
	let mut var_indexes: HashMap<String, usize> = HashMap::new();

	for c in formula.chars()
	{
		if c.is_ascii_uppercase()
		{
			if !var_indexes.contains_key(c.to_string())
			{
				var_indexes.insert(c.to_string(), var_indexes.len());
			}
		}
	}
	if var_indexes.len() == 0
	{
		let ret = eval_formula_opt(formula);
		if ret.is_some()
		{
			println!("Constant formula, evaluate to {}", ret.unwrap());
		}
		return;
	}
	i = 0;
	print!("|");
	for (key, val) in var_indexes
	{
		print!(" {} |", key);
	}
	println!(" = |");
	print!("|");
	for (key, val) in var_indexes
	{
		print!("---|");
	}
	println!("---|");
	while i < 1<<var_indexes.len()
	{
		print!("|");
		for (key, val) in var_indexes
		{
			print!(" {} |", key);
		}
		let ret = execute_formula(formula, &var_indexes, i);
		if ret.is_none()
		{
			println!(" ! |");
		}
		else if ret.unwrap()
		{
			println!(" 1 |");
		}
		else
		{
			println!(" 0 |");
		}
	}
}

fn	execute_formula(
	formula: &str,
	var_indexes: &HashMap<String, usize>,
	values: u32
) -> Option<bool>
{
	let mut moded_formula = formula;
	for (key, val) in var_indexes
	{
		let char_to_put = if (values >> val & 1) == 1 {"1"} else {"0"};
		moded_formula = &moded_formula.replace(key, char_to_put);
	}
	return eval_formula_opt(moded_formula);
}
