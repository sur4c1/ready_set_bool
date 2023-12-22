/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   conjunction_normal_form.rs                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/21 17:40:00 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/22 17:40:45 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::negation_normal_form::nnf_tree;
use crate::exs::negation_normal_form::parse_tree;
use crate::exs::negation_normal_form::tree_to_string;
use crate::exs::negation_normal_form::EquationTree;

pub fn conjunction_normal_form(formula: &str) -> String {
	let mut tree = parse_tree(formula);
	if tree.is_none() {
		eprintln!("Error: Invalid formula");
		return "INVALID FORMULA".to_string();
	}
	nnf_tree(&mut tree);
	cnf_tree(&mut tree);
	return tree_to_string(&tree.unwrap());
}

pub fn cnf_tree(tree: &mut Option<Box<EquationTree>>) {
	while let Some(illegal) = find_illegal(tree) {
		let left = illegal.left.take();
		let right = illegal.right.take();

		if left.is_none() || right.is_none() {
			eprintln!("Error: Unexpected empty children");
			return;
		}
		let left = left.unwrap();
		let mut right = right.unwrap();

		// If left child is a operator and right child is not
		if (left.value == "&" || left.value == "|") && !(right.value == "&" || right.value == "|") {
			let new_left = Some(Box::new((*right).clone()));
			let new_right = Some(Box::new((*left).clone()));
			// Swap left and right
			illegal.set_left(new_left);
			illegal.set_right(new_right);
			continue;
		}

		// If disjunction contains a conjunction
		// A | (B & C) => (A | B) & (A | C)
		if illegal.value == "|" && right.value == "&" {
			let mut new_left = EquationTree::new("|".to_string());
			let mut new_right = EquationTree::new("|".to_string());
			let b = right.left.take();
			let c = right.right.take();

			new_left.set_left(Some(Box::new((*left).clone())));
			new_left.set_right(b);
			new_right.set_left(Some(Box::new((*left).clone())));
			new_right.set_right(c);

			illegal.value = "&".to_string();
			illegal.set_left(Some(Box::new(new_left)));
			illegal.set_right(Some(Box::new(new_right)));
		}
	}
}

fn find_illegal<'a>(tree: &'a mut Option<Box<EquationTree>>) -> Option<&'a mut Box<EquationTree>> {
	if let Some(node) = tree {
		// If the left child is an operator and the right child is not
		if (node.value == "&" || node.value == "|")
			&& (node.left.as_ref().unwrap().value == "&"
				|| node.left.as_ref().unwrap().value == "|")
			&& !(node.right.as_ref().unwrap().value == "&"
				|| node.right.as_ref().unwrap().value == "|")
		{
			return Some(node);
		}
		// If disjunction contains a conjunction
		if node.value == "|"
			&& (node.left.as_ref().unwrap().value == "&"
				|| node.right.as_ref().unwrap().value == "&")
		{
			return Some(node);
		}
		if let Some(illegal) = find_illegal(&mut node.left) {
			return Some(illegal);
		}
		if let Some(illegal) = find_illegal(&mut node.right) {
			return Some(illegal);
		}
		return None;
	} else {
		return None;
	}
}
