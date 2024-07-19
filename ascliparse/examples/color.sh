#!/usr/bin/env bash
# @author      : denstiny (2254228017@qq.com)
# @file        : color
# @created     : Tuesday Jul 02, 2024 13:29:06 CST 
# @github      : https://github.com/denstiny
# @blog        : https://denstiny.github.io

# 定义 ANSI 颜色转义序列
RED="\e[38;2;255;0;0m"
GREEN="\e[38;2;0;255;0m"
BLUE="\e[38;2;0;0;255m"
YELLOW="\e[38;2;255;255;0m"
MAGENTA="\e[38;2;255;0;255m"
CYAN="\e[38;2;0;255;255m"
RESET="\e[0m"

# 打印颜色
echo -e "${RED}Red${RESET}"
echo -e "${GREEN}Green${RESET}"
echo -e "${BLUE}Blue${RESET}"
echo -e "${YELLOW}Yellow${RESET}"
echo -e "${MAGENTA}Magenta${RESET}"
echo -e "${CYAN}Cyan${RESET}"

