#include <stdio.h>
#include <stdlib.h>

#ifndef ERROR_ARGUMENTS
#define ERROR_ARGUMENTS 2
#endif

int main(int argc, char *argv[]) {
    char *end_ptr;
    unsigned int multiplier;
    unsigned int multiplicand;
    const char *output_type;

    if (argc != 4) {
        printf("Content-Type: text/plain;charset=UTF-8\n\n");
        printf("Error: Some arguments are missing.\n");
        if (argc < 2) {
            printf("The argument #1 is missing.\n");
        }
        if (argc < 3) {
            printf("The argument #2 is missing.\n");
        }
        if (argc < 4) {
            printf("The argument #3 is missing.\n");
        }
        if (argc > 4) {
            printf("Too many arguments supplied.\n");
        }
        printf("Exiting...\n");
        exit(ERROR_ARGUMENTS);
    }

    multiplier = strtol(argv[1], &end_ptr, 10);
    multiplicand = strtol(argv[2], &end_ptr, 10);
    output_type = argv[3];

    printf("Content-Type: text/%s;charset=UTF-8\n\n", output_type);

    printf("multiplier:   %d\n", multiplier);
    printf("multiplicand: %d\n", multiplicand);
    printf("output_type:  %s\n", output_type);

    return 0;
}
