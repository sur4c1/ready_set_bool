/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_formula.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 15:40:56 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/13 11:18:44 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::collections::VecDeque;

pub fn eval_formula(formula: &str) -> bool {
	return eval_formula_opt(formula).unwrap_or(false);
}

pub fn eval_formula_opt(formula: &str) -> Option<bool> {
	let mut stack: VecDeque<bool> = VecDeque::new();
	let mut i: usize = 0;

	while i < formula.len() {
		let c = formula.chars().nth(i).unwrap();
		if c == '0' || c == '1' {
			stack.push_back(c == '1');
			i += 1;
			continue;
		}
		if !bool_operators().contains(c) {
			eprintln!("eval_formula: {} is not a valid symbol", c);
			return None;
		}
		if c == '!' {
			let tmp = stack.pop_back().unwrap();
			stack.push_back(!tmp);
			i += 1;
			continue;
		}
		if stack.len() < 2 {
			eprintln!("eval_formula: {} requiries 2 operands, find one", c);
			return None;
		}
		if c == '&' {
			let a = stack.pop_back().unwrap();
			let b = stack.pop_back().unwrap();
			stack.push_back(a && b);
		}
		if c == '|' {
			let a = stack.pop_back().unwrap();
			let b = stack.pop_back().unwrap();
			stack.push_back(a || b);
		}
		if c == '^' {
			let a = stack.pop_back().unwrap();
			let b = stack.pop_back().unwrap();
			stack.push_back(a != b);
		}
		if c == '>' {
			let a = stack.pop_back().unwrap();
			let b = stack.pop_back().unwrap();
			stack.push_back(!a || b);
		}
		if c == '=' {
			let a = stack.pop_back().unwrap();
			let b = stack.pop_back().unwrap();
			stack.push_back(a == b);
		}
		i += 1;
	}
	if stack.len() != 1 {
		eprintln!("eval_formula: {} is not a valid formula", formula);
		return None;
	} else {
		return Some(stack.pop_back().unwrap());
	}
}

fn bool_operators() -> &'static str {
	return "!&|^>=";
}
