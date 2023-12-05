/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2023/12/05 14:56:42 by bguyot            #+#    #+#             */
/*   Updated: 2023/12/05 15:23:41 by bguyot           ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
use std::io::Write;

mod	tests;
mod	exs;

fn main() {
	let mut lines = io::stdin().lines();

	println!("Hi!");
	show_prompt();
	while let Some(line) = lines.next()
	{
		let cmd = line.unwrap();

		if cmd == "00" || cmd == "0"
		{
			tests::ex00::ex00();
		}
		else if cmd == "01" || cmd == "1"
		{
			tests::ex01::ex01();
		}
		else if cmd == "02" || cmd == "2"
		{
			tests::ex02::ex02();
		}
		else if cmd == "Q" || cmd == "q" || cmd == "quit" || cmd == "exit"
		{
			println!("Exiting...");
			break;
		}
		else
		{
			println!("Bad input");
		}
		show_prompt();
	}
	println!("Bye!");
}

fn show_prompt()
{
	println!("\nChoisir exercice à tester: (00-11)");
	print!("> ");
	io::stdout().flush().unwrap();
}