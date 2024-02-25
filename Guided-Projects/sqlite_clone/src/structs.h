#include "external_libs.h"
#include "constants.h"

typedef struct {
  char* buffer;
  size_t buffer_length;
  ssize_t input_length;
} InputBuffer;

typedef struct {
  StatementType type;
} Statement;
