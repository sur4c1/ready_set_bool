/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   negation_normal_form.rs                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/14 11:25:46 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/21 18:02:10 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub struct	EquationTree
{
	pub left: Option<Box<EquationTree>>,
	pub right: Option<Box<EquationTree>>,
	pub value: String,
}

impl EquationTree {
	// Constructor to create a new EquationTree with a given value
	pub fn new(value: String) -> Self {
		EquationTree {
			value,
			left: None,
			right: None,
		}
	}

	// Function to set the left child by taking ownership of another EquationTree
	pub fn set_left(&mut self, new_left: Option<Box<EquationTree>>) {
		self.left = new_left;
	}

	// Function to set the right child by taking ownership of another EquationTree
	pub fn set_right(&mut self, new_right: Option<Box<EquationTree>>) {
		self.right = new_right;
	}

	// Clone the EquationTree
	pub fn clone(&self) -> Self {
		EquationTree {
			value: self.value.clone(),
			left: match &self.left {
				Some(left) => Some(Box::new((*left).clone())),
				None => None,
			},
			right: match &self.right {
				Some(right) => Some(Box::new((*right).clone())),
				None => None,
			},
		}
	}
}

pub fn	nnf_tree(tree: &mut Option<Box<EquationTree>>) {
	while let Some(illegal) = find_illegal(tree)
	{
		// XOR (a ^ b) = (a | b) & (!a & !b) NOTE: using the CNF directly
		if illegal.value == "^"
		{
			if let (Some(left), Some(right))
				= (&illegal.left, &illegal.right)
			{
				let mut not_left = EquationTree::new("!".to_string());
				not_left.set_left(Some(Box::new((*left).clone())));

				let mut not_right = EquationTree::new("!".to_string());
				not_right.set_left(Some(Box::new((*right).clone())));

				let mut new_left = EquationTree::new("|".to_string());
				new_left.set_left(Some(Box::new((*left).clone())));
				new_left.set_right(Some(Box::new((*right).clone())));

				let mut new_right = EquationTree::new("|".to_string());
				new_right.set_left(Some(Box::new(not_left)));
				new_right.set_right(Some(Box::new(not_right)));

				illegal.value = "&".to_string();
				illegal.set_left(Some(Box::new(new_left)));
				illegal.set_right(Some(Box::new(new_right)));
			}
		}
		// IMPLIES (a > b) = (!a | b)
		else if illegal.value == ">"
		{
			if let (Some(left), Some(right))
				= (&illegal.left, &illegal.right)
			{
				let mut not_left = EquationTree::new("!".to_string());
				not_left.set_left(Some(Box::new((*left).clone())));

				illegal.value = "|".to_string();
				illegal.set_right(Some(Box::new((*right).clone())));
				illegal.set_left(Some(Box::new(not_left)));
			}
		}
		// EQUIVALENCE (a = b) = (a & b) | (!a & !b)
		else if illegal.value == "="
		{
			if let (Some(left), Some(right))
				= (&illegal.left, &illegal.right)
			{
				let mut not_left = EquationTree::new("!".to_string());
				not_left.set_left(Some(Box::new((*left).clone())));

				let mut not_right = EquationTree::new("!".to_string());
				not_right.set_left(Some(Box::new((*right).clone())));

				let mut new_left = EquationTree::new("&".to_string());
				new_left.set_left(Some(Box::new((*left).clone())));
				new_left.set_right(Some(Box::new((*right).clone())));

				let mut new_right = EquationTree::new("&".to_string());
				new_right.set_left(Some(Box::new(not_left)));
				new_right.set_right(Some(Box::new(not_right)));

				illegal.value = "|".to_string();
				illegal.set_left(Some(Box::new(new_left)));
				illegal.set_right(Some(Box::new(new_right)));
			}
		}
		// NOT
		else if illegal.value == "!"
		{
			if let Some(next) = &illegal.left
			{
				// NOT
				if next.value == "!"
				{
					if let Some(next_next) = &next.left
					{
						let (next_next_left,
							next_next_right)
							= (&next_next.left, &next_next.right);
						illegal.value = next_next.value.clone();
						let new_left = if next_next_left.is_some()
						{
							Some(Box::new((*next_next_left.as_ref().unwrap()).clone()))
						}
						else
						{
							None
						};
						let new_right = if next_next_right.is_some()
						{
							Some(Box::new((*next_next_right.as_ref().unwrap()).clone()))
						}
						else
						{
							None
						};
						illegal.set_left(new_left);
						illegal.set_right(new_right);
					}
				}
				// AND
				else if next.value == "&"
				{
					if let (Some(next_left), Some(next_right))
						= (&next.left, &next.right)
					{
						let mut not_left = EquationTree::new("!".to_string());
						not_left.set_left(Some(Box::new((*next_left).clone())));

						let mut not_right = EquationTree::new("!".to_string());
						not_right.set_left(Some(Box::new((*next_right).clone())));

						illegal.value = "|".to_string();
						illegal.set_left(Some(Box::new(not_left)));
						illegal.set_right(Some(Box::new(not_right)));
					}
				}
				// OR
				else if next.value == "|"
				{
					if let (Some(next_left), Some(next_right))
						= (&next.left, &next.right)
					{
						let mut not_left = EquationTree::new("!".to_string());
						not_left.set_left(Some(Box::new((*next_left).clone())));

						let mut not_right = EquationTree::new("!".to_string());
						not_right.set_left(Some(Box::new((*next_right).clone())));

						illegal.value = "&".to_string();
						illegal.set_left(Some(Box::new(not_left)));
						illegal.set_right(Some(Box::new(not_right)));
					}
				}
			}
		}
	}
}

pub fn	negation_normal_form(formula: &str) -> String
{
	let mut tree = parse_tree(formula);
	if tree.is_none() {
		eprintln!("Error: Invalid formula");
		return String::from("INVALID FORMULA");
	}
	nnf_tree(&mut tree);
	return tree_to_string(&tree.unwrap());
}


pub fn tree_to_string(tree: &Box<EquationTree>) -> String {
	if tree.left.is_none() && tree.right.is_none() {
		return tree.value.clone();
	} else if tree.left.is_some() && tree.right.is_none() {
		let mut ret = tree_to_string(&tree.left.as_ref().unwrap());
		ret.push_str(tree.value.as_str());
		return ret;
	} else {
		let mut ret = tree_to_string(&tree.left.as_ref().unwrap());
		ret.push_str(tree_to_string(&tree.right.as_ref().unwrap()).as_str());
		ret.push_str(tree.value.as_str());
		return ret;
	}
}

pub fn parse_tree(formula: &str) -> Option<Box<EquationTree>> {
	let mut stack: Vec<Option<Box<EquationTree>>> = Vec::new();
	for c in formula.chars()
	{
		if c.is_ascii_uppercase()
		{
			stack.push(Some(Box::new(EquationTree::new(c.to_string()))));
		}
		else if c == '!'
		{
			let left = stack.pop();
			if left.is_none()
			{
				return None;
			}
			let mut tree = EquationTree::new(c.to_string());
			tree.set_left(left.unwrap());
			stack.push(Some(Box::new(tree)));
		}
		else if c == '&' || c == '|' || c == '>' || c == '=' || c == '^'
		{
			let right = stack.pop();
			let left = stack.pop();
			if left.is_none() || right.is_none()
			{
				return None;
			}
			let mut tree = EquationTree::new(c.to_string());
			tree.set_left(left.unwrap());
			tree.set_right(right.unwrap());
			stack.push(Some(Box::new(tree)));
		}
		else
		{
			return None;
		}
	}
	if stack.len() != 1
	{
		return None;
	}
	return stack.pop().unwrap();
}

fn find_illegal<'a>(
	tree: &'a mut Option<Box<EquationTree>>
) -> Option<&'a mut Box<EquationTree>> {
	if let Some(tree_ref) = tree.as_mut() {
		// Check if the current node is illegal
		if tree_ref.value == "^"
			|| tree_ref.value == "="
			|| tree_ref.value == ">" || (
				tree_ref.value == "!"
				&& (tree_ref.left.as_ref().unwrap().value == "&"
					|| tree_ref.left.as_ref().unwrap().value == "|"
					|| tree_ref.left.as_ref().unwrap().value == "!"
				)
			)
		{
			return Some(tree_ref);
		}

		// Check if the left or right child is illegal
		if let Some(illegal) = find_illegal(&mut tree_ref.left) {
			return Some(illegal);
		}
		if let Some(illegal) = find_illegal(&mut tree_ref.right) {
			return Some(illegal);
		}
	}
	// No illegal node found
	None
}
