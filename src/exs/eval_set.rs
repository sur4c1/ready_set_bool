/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   eval_set.rs                                        :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/27 11:13:07 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/27 11:38:47 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

pub fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>
{
	let mut stack = Vec::new();
	let mut omega = Vec::new();

	// Fill omega with the union of all sets
	for set in sets.iter()
	{
		for elem in set.iter()
		{
			if !omega.contains(elem)
			{
				omega.push(*elem);
			}
		}
	}

	// Evaluate the formula as a stack (RPN)
	for c in formula.chars()
	{
		if c.is_ascii_uppercase()
		{
			let index = (c as u8 - 'A' as u8) as usize;
			if index >= sets.len()
			{
				eprintln!("eval_set: set {} not found", c);
				return vec![];
			}
			stack.push(sets[index].clone());
		}
		else if c == '!'
		{
			let temp = stack.pop();
			if temp.is_none()
			{
				eprintln!("eval_set: invalid formula");
				return vec![];
			}
			let temp = temp.unwrap();
			let mut inversed = Vec::new();
			for elem in omega.iter()
			{
				if !temp.contains(elem)
				{
					inversed.push(*elem);
				}
			}
			stack.push(inversed);
		}
		else if c == '&' || c == '|' || c == '^' || c == '=' || c == '>'
		{
			let right = stack.pop();
			let left = stack.pop();
			if left.is_none() || right.is_none()
			{
				eprintln!("eval_set: invalid formula: {} is missing an operand", c);
				return vec![];
			}
			let right = right.unwrap();
			let left = left.unwrap();
			let mut new_set = Vec::new();

			// Intersection
			if c == '&'
			{
				for elem in omega.iter()
				{
					if left.contains(elem) && right.contains(elem)
					{
						new_set.push(*elem);
					}
				}
			}
			// Union
			else if c == '|'
			{
				for elem in omega.iter()
				{
					if left.contains(elem) || right.contains(elem)
					{
						new_set.push(*elem);
					}
				}
			}
			// Symmetric difference
			else if c == '^'
			{
				for elem in omega.iter()
				{
					if (left.contains(elem) && !right.contains(elem))
						|| (!left.contains(elem) && right.contains(elem))
					{
						new_set.push(*elem);
					}
				}
			}
			// Logical equality (https://en.wikipedia.org/wiki/Logical_equality)
			else if c == '='
			{
				for elem in omega.iter()
				{
					if (left.contains(elem) && right.contains(elem))
						|| (!left.contains(elem) && !right.contains(elem))
					{
						new_set.push(*elem);
					}
				}
			}
			// Material implication (https://en.wikipedia.org/wiki/Material_conditional)
			else if c == '>'
			{
				for elem in omega.iter()
				{
					if !left.contains(elem) || right.contains(elem)
					{
						new_set.push(*elem);
					}
				}
			}
			stack.push(new_set);
		}
	}

	if stack.len() != 1
	{
		eprintln!("eval_set: invalid formula");
		return vec![];
	}

	return stack.pop().unwrap();
}
