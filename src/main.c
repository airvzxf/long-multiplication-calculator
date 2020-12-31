#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#ifndef ERROR_ARGUMENTS
#define ERROR_ARGUMENTS 2
#endif

struct Multiplication {
    unsigned long long int multiplier;
    const char *multiplier_str;
    unsigned long long int multiplier_size;

    unsigned long long int multiplicand;
    const char *multiplicand_str;
    unsigned long long int multiplicand_size;

    unsigned long long int result;
    unsigned long long int result_size;

// Decrease the size of the array if the exit code is 139 (interrupted by signal 11: SIGSEGV).
    unsigned long long int operations[500000];
    unsigned long long int operations_size;

    unsigned char is_printing_description;
};

int count_digits(unsigned long long int number);

void print_multiplication_separator(unsigned long long int repeat);

void print_multiplication_custom_separator(unsigned long long int repeat, char separator);

void set_text_fill(char **text, char character, unsigned long long int size);

void print_text(struct Multiplication *m);

void generate_operations(struct Multiplication *m);

unsigned long long int get_power(unsigned long long int base, unsigned long long int exponent);

int count_digits(unsigned long long int number) {
    if (number == 0) return 1;

    int count = 0;
    while (number != 0) {
        number = number / 10;
        ++count;
    }
    return count;
}


void print_multiplication_custom_separator(unsigned long long int repeat, char separator) {
    for (unsigned long long int i = 0; i < repeat; i++) {
        printf("%c", separator);
    }

    printf("\n");
}

void print_multiplication_separator(unsigned long long int repeat) {
    print_multiplication_custom_separator(repeat, '-');
}

void set_text_fill(char **text, char character, unsigned long long int size) {
    char characters[4000];

    for (unsigned long long int i = 0; i <= size; i++) {
        characters[i] = character;
        if (i == size) {
            characters[i] = '\000';
        }
    }

    *text = malloc(size);
    strcpy(*text, characters);
}

void print_text(struct Multiplication *m) {
    const unsigned long long int spaces_left = 2;
    const unsigned long long int separator = spaces_left + m->result_size;
    unsigned long long int sub_spaces_left;
    char *spaces_str, *zeros_str, *description_str;

    sub_spaces_left = spaces_left + (m->result_size - m->multiplicand_size);
    set_text_fill(&spaces_str, ' ', sub_spaces_left);
    printf("%s%s", spaces_str, m->multiplicand_str);
    if (m->is_printing_description) printf(" ---> Multiplicand => a");
    printf("\n");

    sub_spaces_left = spaces_left - 1 + (m->result_size - m->multiplier_size);
    set_text_fill(&spaces_str, ' ', sub_spaces_left);
    printf("x%s%s", spaces_str, m->multiplier_str);
    if (m->is_printing_description) printf(" ---> Multiplier   => b");
    printf("\n");

    print_multiplication_custom_separator(separator, '=');

    unsigned long long int spaces_inner_left, pre_space, actual_multiplier;
    for (unsigned long long int i = 0; i < m->operations_size; i += 3) {
        pre_space = m->result_size - (i / 3);
        actual_multiplier = (i / 3) + 1;
        set_text_fill(&description_str, ' ', (i / 3));

        spaces_inner_left = pre_space - count_digits(m->operations[i]);
        set_text_fill(&spaces_str, ' ', spaces_inner_left);
        printf("  %s%llu", spaces_str, m->operations[i]);
        if (m->is_printing_description) {
            printf(" %s---> First digit: b%llu * a[x]", description_str, actual_multiplier);
        }
        printf("\n");

        spaces_inner_left = pre_space - count_digits(m->operations[i + 1]);
        set_text_fill(&spaces_str, ' ', spaces_inner_left);
        printf("+ %s%llu", spaces_str, m->operations[i + 1]);
        if (m->is_printing_description) {
            printf(" %s---> Carry: b%llu * a[x]", description_str, actual_multiplier);
        }
        printf("\n");

        spaces_inner_left = pre_space - count_digits(m->operations[i + 2]);
        set_text_fill(&spaces_str, ' ', spaces_inner_left);
        printf("= %s%llu", spaces_str, m->operations[i + 2]);
        if (m->is_printing_description) printf(" %s---> Result of the sum", description_str);
        printf("\n");

        if (i + 3 < m->operations_size) {
            print_multiplication_separator(separator);
        }
    }

    print_multiplication_custom_separator(separator, '=');

    for (unsigned long long int i = 2; i < m->operations_size; i += 3) {
        pre_space = m->result_size - (i / 3);
        actual_multiplier = (i / 3) + 1;
        spaces_inner_left = pre_space - count_digits(m->operations[i]);
        set_text_fill(&spaces_str, ' ', spaces_inner_left);
        set_text_fill(&zeros_str, '0', (i / 3));
        printf("+ %s%llu%s", spaces_str, m->operations[i], zeros_str);
        if (m->is_printing_description) printf(" ---> Result: b%llu * a", actual_multiplier);
        printf("\n");
    }

    print_multiplication_separator(separator);
    printf("= %llu", m->result);
    if (m->is_printing_description) printf(" ---> Final result");
    printf("\n");
}

void generate_operations(struct Multiplication *m) {
    unsigned long long int multiplier, multiplicand;
    unsigned long long int multiplication;
    unsigned long long int first_digits;
    unsigned long long int last_digit;
    unsigned long long int digit_position, carry_position;
    unsigned long long int index = 0;
    unsigned long long int index_multiplier;
    unsigned long long int index_multiplicand;

    const unsigned long long int multiplier_max = m->multiplier_size - 1;
    const unsigned long long int multiplicand_max = m->multiplicand_size - 1;

    for (unsigned long long int a = 0; a <= multiplier_max; a++) {
        index_multiplier = multiplier_max - a;
        index = (multiplier_max - index_multiplier) * 3;
        m->operations[index + 1] += 0;

        for (unsigned long long int b = 0; b <= multiplicand_max; b++) {
            index_multiplicand = multiplicand_max - b;
            digit_position = get_power(10, b);
            carry_position = get_power(10, (b + 1));
            multiplier = (unsigned long long int) m->multiplier_str[index_multiplier] - 48;
            multiplicand = (unsigned long long int) m->multiplicand_str[index_multiplicand] - 48;
            multiplication = multiplier * multiplicand;
            last_digit = multiplication % 10;
            first_digits = multiplication / 10;
            m->operations[index] += last_digit * digit_position;
            m->operations[index + 1] += first_digits * carry_position;
        }

        m->operations[index + 2] = m->operations[index] + m->operations[index + 1];
    }

    m->operations_size = index + 3;
}

unsigned long long int get_power(unsigned long long int base, unsigned long long int exponent) {
    unsigned long long int result = 1;

    for (int i = 0; i < exponent; i++) {
        result *= base;
    }

    return result;
}

int main(int argc, char *argv[]) {
    if (argc != 5) {
        printf("Content-Type: text/plain;charset=UTF-8\n\n");
        printf("Error: Some arguments are missing.\n");
        if (argc < 2) {
            printf("The argument #1 (multiplier) is missing.\n");
        }
        if (argc < 3) {
            printf("The argument #2 (multiplicand) is missing.\n");
        }
        if (argc < 4) {
            printf("The argument #3 (output_type) is missing.\n");
        }
        if (argc < 5) {
            printf("The argument #4 (print_description) is missing.\n");
        }
        if (argc > 5) {
            printf("Too many arguments supplied.\n");
        }
        printf("Exiting...\n");
        exit(ERROR_ARGUMENTS);
    }

    const char *output_type;
    struct Multiplication multiplication;

    output_type = argv[3];
    printf("Content-Type: text/%s;charset=UTF-8\n\n", output_type);

    char *end_ptr;
    multiplication.multiplier = strtoul(argv[1], &end_ptr, 10);
    multiplication.multiplicand = strtoul(argv[2], &end_ptr, 10);

    multiplication.multiplier_str = argv[1];
    multiplication.multiplicand_str = argv[2];
    multiplication.multiplier_size = strlen(multiplication.multiplier_str);
    multiplication.multiplicand_size = strlen(multiplication.multiplicand_str);

    char result_str[4000];
    multiplication.result = multiplication.multiplier * multiplication.multiplicand;
    sprintf(result_str, "%llu", multiplication.result);
    multiplication.result_size = strlen(result_str);

    if (argv[4][0] == 'n' || argv[4][0] == 'N') {
        multiplication.is_printing_description = 0;
    }

    generate_operations(&multiplication);

    print_text(&multiplication);

    return 0;
}
