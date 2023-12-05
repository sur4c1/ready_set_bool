# **************************************************************************** #
#                                                                              #
#                                                         :::      ::::::::    #
#    Makefile                                           :+:      :+:    :+:    #
#                                                     +:+ +:+         +:+      #
#    By: bguyot <bguyot@student.42mulhouse.fr>      +#+  +:+       +#+         #
#                                                 +#+#+#+#+#+   +#+            #
#    Created: 2023/12/05 10:48:13 by bguyot            #+#    #+#              #
#    Updated: 2023/12/05 11:15:31 by bguyot           ###   ########.fr        #
#                                                                              #
# **************************************************************************** #

RM	=	rm -f

all: ex00

EX00_DIR		=	ex00_adder
EX00_SRC_FILES	=	adder
EX00_SRCS		=																\
	$(addprefix $(EX00_DIR)/, $(addsuffix .rs, $(EX00_SRC_FILES)))
EX00_OBJS		=	$(addsuffix .o, $(EX00_SRC_FILES))
EX00_MAIN		=	$(addprefix $(EX00_DIR)/, $(addsuffix .rs, main))
ex00: $(EX00_MAIN) $(EX00_SRCS)
	rustc $(EX00_MAIN) -o ex00

clean:

fclean: clean
	$(RM) ex00
