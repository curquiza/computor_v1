#!/bin/bash

GREEN="\033[1;32m"
RED="\033[1;31m"
DEF="\033[0m"

answer_degree0_no="There is no solution"
answer_degree0_all="All real numbers are solution"
answer_degree1="There is an unique solution"
answer_degree2_zero="There is an unique solution"
answer_degree2_pos="There is two solutions."
answer_degree2_neg="There is two complexe solutions"

# run_test_0_no_sol "equation"
function run_test_0_no_sol() {
    local output=$(cargo run -- "$1" 2>&1)
    local answer=$(echo "$output" | grep "$answer_degree0_no")
    if [[ "$answer" != "" ]]; then
        printf "%-50s$GREEN%s$DEF\n" "$1" "OK"
    else
        printf "%-50s$RED%s$DEF\n" "$1" "KO"
    fi
}

# run_test_0_all_sol "equation"
function run_test_0_all_sol() {
    local output=$(cargo run -- "$1" 2>&1)
    local answer=$(echo "$output" | grep "$answer_degree0_all")
    if [[ "$answer" != "" ]]; then
        printf "%-50s$GREEN%s$DEF\n" "$1" "OK"
    else
        printf "%-50s$RED%s$DEF\n" "$1" "KO"
    fi
}

# run_test_1 "equation" "solution"
function run_test_1() {
    local output=$(cargo run -- "$1" 2>&1)
    local answer=$(echo "$output" | grep "$answer_degree1")
    local solution=$(echo "$output" | grep "Solution" | awk -F "=" '{print $2}' | tr -d " ")
    if [[ "$answer" != "" ]] && [[ "$solution" == "$2" ]]; then
        printf "%-50s$GREEN%s$DEF\n" "$1" "OK"
    else
        printf "%-50s$RED%s$DEF\n" "$1" "KO"
    fi
}

# run_test_2_zero "equation" "solution"
function run_test_2_zero() {
    local output=$(cargo run -- "$1" 2>&1)
    local answer=$(echo "$output" | grep "$answer_degree2_zero")
    local solution=$(echo "$output" | grep "Solution" | awk -F "=" '{print $3}' | tr -d " ")
    if [[ "$answer" != "" ]] && [[ "$solution" == "$2" ]]; then
        printf "%-50s$GREEN%s$DEF\n" "$1" "OK"
    else
        printf "%-50s$RED%s$DEF\n" "$1" "KO"
    fi
}

# run_test_2_pos "equation" "solution1" "solution2"
function run_test_2_pos() {
    local output=$(cargo run -- "$1" 2>&1)
    local answer=$(echo "$output" | grep "$answer_degree2_pos")
    local solution1=$(echo "$output" | grep "Solution 1" | awk -F "=" '{print $3}' | tr -d " ")
    local solution2=$(echo "$output" | grep "Solution 2" | awk -F "=" '{print $3}' | tr -d " ")
    if [[ "$answer" != "" ]] && [[ "$solution1" == "$2" ]] && [[ "$solution2 == "$3"" ]]; then
        printf "%-50s$GREEN%s$DEF\n" "$1" "OK"
    else
        printf "%-50s$RED%s$DEF\n" "$1" "KO"
    fi
}

# run_test_2_neg "equation" "solution1" "solution2"
function run_test_2_neg() {
    local output=$(cargo run -- "$1" 2>&1)
    local answer=$(echo "$output" | grep "$answer_degree2_neg")
    local solution1=$(echo "$output" | grep "Solution 1" | awk -F "=" '{print $4}' | tr -d " ")
    local solution2=$(echo "$output" | grep "Solution 2" | awk -F "=" '{print $4}' | tr -d " ")
    if [[ "$answer" != "" ]] && [[ "$solution1" == "$2" ]] && [[ "$solution2 == "$3"" ]]; then
        printf "%-50s$GREEN%s$DEF\n" "$1" "OK"
    else
        printf "%-50s$RED%s$DEF\n" "$1" "KO"
    fi
}

echo "*** DEGREE 0 ***"
run_test_0_all_sol "42 * X^1 = 42 * X"
run_test_0_all_sol "42 * X^0 = 42 * X^0"
run_test_0_no_sol  "0 * X^453 = 5"
run_test_0_all_sol "0 * X^453 = 0"
run_test_0_all_sol "5 * X^0 = 5 * X^0"
run_test_0_all_sol "5 * X^0 + X^2 = 5 * X^0 + X^2"
run_test_0_no_sol "5 * X^0 + X^2 = 5 * X^0 + X^2 + 1"
run_test_0_no_sol "4 * X^0 = 8 * X^0"

echo "\n*** DEGREE 1 ***"
run_test_1 "X + 1 = 0"                          "-1"
run_test_1 "X = 0"                              "0"
run_test_1 "2 * X = -45"                        "-22.5"
run_test_1 "2 * X + 3 = -45 * X + 3 - -9"       "0.19148936170212766"
run_test_1 "-12.5 * X^1 - 3.4665 = 0 * X^2 - X" "-0.30143478260869566"
run_test_1 "5 * X^0 + 4 * X^1 = 4 * X^0"        "-0.25"
run_test_1 "5 + 4 * X + X^2 = X^2"              "-1.25"
run_test_1 "5 * X^0 = 4 * X^0 + 7 * X^1"        "0.14285714285714285"

echo "\n*** DEGREE 2 ***"
echo "* delta = "
run_test_2_zero "X^2 + 2 * X + 1 = 0"                              "-1"
run_test_2_zero "-2 * X^2 + 2 * X + 1 = -2 * X^2 + X^2 + 2"        "1"
run_test_2_zero "6 * X^0 + 11 * X^1 + 5 * X^2 = 1 * X^0 + 1 * X^1" "-1"
echo "* delta > 0"
run_test_2_pos "X^2 + 34 * X + 1 = 0"                             "-0.029437251522857366" "-33.97056274847714"
run_test_2_pos "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"          "-0.47513146390886934"  "0.9052389907905898"
run_test_2_pos "5 * X^0 + 13 * X^1 + 3 * X^2 = 1 * X^0 + 1 * X^1" "-0.3670068381445481"   "-3.632993161855452"
echo "* delta < 0"
run_test_2_neg "X^2 + X + 1 = 0" "-0.5+i*0.8660254037844386" "-0.5-i*0.8660254037844386"
run_test_2_neg "3 * X^0 + 1 * X^1 = 0 * X^0 - 1 * X^1 - 3 * X^2" "-0.3333333333333333+i*0.9428090415820632" "-0.3333333333333333-i*0.9428090415820632"
run_test_2_neg "5 * X^0 + 3 * X^1 + 3 * X^2 = 1 * X^0 + 0 * X^1" "-0.5+i*1.0408329997330663" "0.5-i*1.0408329997330663"
