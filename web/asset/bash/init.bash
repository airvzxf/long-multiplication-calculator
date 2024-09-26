#!/bin/env bash
set -e

echo "Content-Type: text/plain;charset=UTF-8"
echo ""

EXPECTED_URL_ARGUMENTS=1
if [[ "${#}" -ne "${EXPECTED_URL_ARGUMENTS}" ]]; then
  echo "Error: the parameter in the URL is missing, it should be '${0}?5,79'"
  echo "       Parameter #1 is the multiplier:    5"
  echo "       Parameter #2 is the multiplicand:  79"
  echo "Total of arguments: ${#} | Expected arguments: ${EXPECTED_URL_ARGUMENTS}"
  echo "Query:"
  echo "   ${0}?${*}"
  echo "Exiting..."
  exit 2
fi

EXPECTED_ARGUMENTS=2
IFS=',' read -r -a ARGUMENTS <<<"${1}"
if [[ "${#ARGUMENTS[*]}" -ne "${EXPECTED_ARGUMENTS}" ]]; then
  echo "Error: some parameters are missing:"
  echo "       Parameter #1 is the multiplier:    ${ARGUMENTS[0]}"
  echo "       Parameter #2 is the multiplicand:  ${ARGUMENTS[1]}"
  echo "Total of arguments: ${#ARGUMENTS[*]} | Expected arguments: ${EXPECTED_ARGUMENTS}"
  echo "Exiting..."
  exit 2
fi

MULTIPLIER="${ARGUMENTS[0]}"
MULTIPLICAND="${ARGUMENTS[1]}"

#EXEC_FILE="../bin/long-multiplication-command-line"
EXEC_FILE="/home/rovisoft/.htpasswds/public_html/projects/multiplication/asset/bin/long-multiplication-command-line"
if [ ! -f "${EXEC_FILE}" ]; then
    echo "Error: file '${EXEC_FILE}' not found!"
    exit 1
fi
chmod 0700 "${EXEC_FILE}"
"${EXEC_FILE}" "${MULTIPLIER}" "${MULTIPLICAND}"
