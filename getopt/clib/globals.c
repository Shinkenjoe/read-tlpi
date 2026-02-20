#include <unistd.h>
#include "globals.h"

const int *get_optopt_location() { return &optopt; }
const int *get_optind_location() { return &optind; }
const int *get_opterr_location() { return &opterr; }
const char* get_optarg_cstr() { return optarg; }
