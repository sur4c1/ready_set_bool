/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   conjunction_normal_form.rs                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/21 17:40:00 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/21 18:18:03 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use crate::exs::negation_normal_form::EquationTree;
use crate::exs::negation_normal_form::nnf_tree;
use crate::exs::negation_normal_form::parse_tree;
use crate::exs::negation_normal_form::tree_to_string;

pub fn conjunction_normal_form(formula: &str) -> String
{
	let mut tree = parse_tree(formula);
	if tree.is_none()
	{
		eprintln!("Error: Invalid formula");
		return "INVALID FORMULA".to_string();
	}
	nnf_tree(&mut tree);
	cnf_tree(&mut tree);
	return tree_to_string(&tree.unwrap());
}

pub fn cnf_tree(tree: &mut Option<Box<EquationTree>>)
{
	// ABC&| -> A|B & A|C (distribute the & over the |)

	// Commutativity of | and &
	// AB|C| -> ABC|| (commutativity of |)
	// AB&C& -> ABC&& (commutativity of &)
		// NOTE: If only one children is a | or &, then we make sure it's the right one
	while let Some(illegal) = find_illegal(tree)
	{
		if illegal.value == "|"
		{
			let left = *illegal.left.take().unwrap();
			let right = *illegal.right.take().unwrap();

			if left.value == "&" || left.value == "|"
			{
				// Swap the left and right children
				// NOTE: I can do this bc | is commutative
				// NOTE: Might not be necessary, but still doing it every time
				illegal.set_left(Some(Box::new(right.clone())));
				illegal.set_right(Some(Box::new(left.clone())));
			}
			{
				// Swap the left and right children
				// NOTE: I can do this bc | is commutative
				illegal.set_left(Some(Box::new(right.clone())));
				illegal.set_right(Some(Box::new(left.clone())));
			}
			// Distribute the A | (B & C) into (A | B) & (A | C)
			let a: EquationTree = *illegal.left.take().unwrap();
			let mut right = *illegal.right.take().unwrap();
			let b = *right.left.take().unwrap();
			let c = *right.right.take().unwrap();

			let mut new_left = EquationTree::new("|".to_string());
			let mut new_right = EquationTree::new("|".to_string());
			new_left.set_left(Some(Box::new(a.clone())));
			new_left.set_right(Some(Box::new(b.clone())));
			new_right.set_left(Some(Box::new(a.clone())));
			new_right.set_right(Some(Box::new(c.clone())));
			illegal.set_left(Some(Box::new(new_left)));
			illegal.set_right(Some(Box::new(new_right)));
			illegal.value = "&".to_string();
		}
		else if  illegal.value == "&"
		{
			let left = *illegal.left.take().unwrap();
			let right = *illegal.right.take().unwrap();

			if left.value == "&" || left.value == "|"
			{
				// Swap the left and right children
				// NOTE: I can do this bc | is commutative
				// NOTE: Might not be necessary, but still doing it every time
				illegal.set_left(Some(Box::new(right.clone())));
				illegal.set_right(Some(Box::new(left.clone())));
			}
		}
	}
}

fn find_illegal<'a>(
	tree: &'a mut Option<Box<EquationTree>>
) -> Option<&'a mut Box<EquationTree>> {
	if let Some(node) = tree {
		if node.value == "|"
			&& (
				node.left.as_ref().unwrap().value == "&"
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
	}
	else { return None; }
}
