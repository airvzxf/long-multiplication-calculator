#!/bin/bash -e

function print_header() {
  echo "Content-Type: text/plain;charset=UTF-8"
  echo ""
}

if [[ "${#}" -ne 1 ]]; then
  print_header
  echo "Error: The parameter is missing should be like 'bash/init.sh?5,79,plain'"
  echo "       Parameter #1 is the multiplier:        5"
  echo "       Parameter #2 is the multiplicand:      79"
  echo "       Parameter #3 is the type of output:    plain"
  echo "       Parameter #4 is the print description: yes"
  echo "Total of arguments: ${#}"
  echo "Query:"
  echo "   ${*}"
  echo "Exiting..."
  exit 2
fi

IFS=',' read -r -a ARGUMENTS <<<"${1}"

MULTIPLIER="${ARGUMENTS[0]}"
MULTIPLICAND="${ARGUMENTS[1]}"
OUTPUT_TYPE="${ARGUMENTS[2]}"
PRINT_DESCRIPTION="${ARGUMENTS[3]}"

chmod +x ./../bin/calculator-long-multiplication
./../bin/calculator-long-multiplication "${MULTIPLIER}" "${MULTIPLICAND}" "${OUTPUT_TYPE}" "${PRINT_DESCRIPTION}"
