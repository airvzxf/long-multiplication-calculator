#!/bin/bash -e

function print_header() {
  echo "Content-Type: text/plain;charset=UTF-8"
  echo ""
}

function is_int() {
  test "$@" -eq "$@" && echo "Good bye!"
}

if [[ "${#}" -ne 1 ]]; then
  print_header
  echo "Error: The parameter is missing should be like 'bash/init.sh?5,79,plain'"
  echo "       The first parameter is the multiplier:     5"
  echo "       The second parameter is the multiplicand:  79"
  echo "       The third parameter is the type of output: plain"
  echo "Exiting..."
  exit 2
fi

IFS=', ' read -r -a ARGUMENTS <<<"${1}"
LENGTH="${#ARGUMENTS[@]}"

if [[ "${LENGTH}" -ne 3 ]]; then
  print_header
  echo "Error: Some arguments are missing."
  if [[ "${LENGTH}" -lt 1 ]]; then
    echo "The argument #1 is missing."
  fi
  if [[ "${LENGTH}" -lt 2 ]]; then
    echo "The argument #2 is missing."
  fi
  if [[ "${LENGTH}" -lt 3 ]]; then
    echo "The argument #3 is missing."
  fi
  if [[ "${LENGTH}" -gt 3 ]]; then
    echo "Too many arguments supplied."
  fi
  echo "Exiting..."
  exit 2
fi

MULTIPLIER="${ARGUMENTS[0]}"
MULTIPLICAND="${ARGUMENTS[1]}"
OUTPUT_TYPE="${ARGUMENTS[2]}"

if ! [[ "${MULTIPLIER}" =~ ^[0-9]+$ ]]; then
  print_header
  echo "Multiplier is not a number: ${MULTIPLIER}."
  exit 3
fi

if ! [[ "${MULTIPLICAND}" =~ ^[0-9]+$ ]]; then
  print_header
  echo "Multiplicand is not a number: ${MULTIPLICAND}."
  exit 3
fi

if [[ "${#OUTPUT_TYPE}" -eq 0 ]]; then
  print_header
  echo "The type of output should not empty."
  exit 3
fi

chmod +x ./../bin/calculator-long-multiplication
./../bin/calculator-long-multiplication "${MULTIPLIER}" "${MULTIPLICAND}" "${OUTPUT_TYPE}"
