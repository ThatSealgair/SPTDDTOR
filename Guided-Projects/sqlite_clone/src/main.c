#include "external_libs.h"
#include "structs.h"
#include <stdlib.h>
#include <sys/types.h>

InputBuffer* new_input_buffer() {
  InputBuffer* input_buffer = (InputBuffer*) calloc(1, sizeof(InputBuffer));
  
  return input_buffer;
}

void print_prompt() {
  printf("DB > ");
}

void close_input_buffer(InputBuffer* input_buffer) {
  free(input_buffer->buffer);
  free(input_buffer);
}

void read_input(InputBuffer* input_buffer) {
  // Read input from stdin
  ssize_t bytes_read = getline(&(input_buffer->buffer), &(input_buffer->buffer_length), stdin);

  // getline returns -1 on failure to read a line, including EOF condition.
  if (bytes_read <= 0) {
    fprintf(stderr, "Error reading input\n");
    exit(EXIT_FAILURE);
  }

  // Remove trailing newline
  input_buffer->input_length = bytes_read - 1;
  input_buffer->buffer[bytes_read - 1] = 0;
}

MetaCommandResult do_meta_command(InputBuffer* input_buffer) {
  if (strncmp(input_buffer->buffer, EXIT_COMMAND, EXIT_COMMAND_SIZE) == 0) {
    exit(EXIT_SUCCESS);
  } else {
    return META_COMMAND_UNRECOGNISED_COMMAND;
  }
}

PrepareResult prepare_statement(InputBuffer* input_buffer, Statement* statement) {
  if (strncmp(input_buffer->buffer, USER_INSERT, USER_INSERT_SIZE) == 0) {
    return PREPARE_SUCCESS;
  }
  
  if (strncmp(input_buffer->buffer, USER_SELECT, USER_SELECT_SIZE) == 0) {
    return PREPARE_SUCCESS;
  }

  return PREPARE_UNRECOGNISED_STATEMENT;
}

void execute_statement(Statement* statement) {
  switch (statement->type) {
    case (STATEMENT_INSERT): {
      printf("This is where we would do an insert.\n");
      break;
    }
    case (STATEMENT_SELECT): {
      printf("This is where we would do a select.\n");
      break;
    }
  }
}

int main(int argc, char** argv) {

  InputBuffer* input_buffer = new_input_buffer();

  while (true) {
    print_prompt();
    read_input(input_buffer);

    if (input_buffer->buffer[0] == META_COMMAND) {
      switch (do_meta_command(input_buffer)) {
        case (META_COMMAND_SUCCESS): {
          continue;
        }
        case (META_COMMAND_UNRECOGNISED_COMMAND): {
          printf("Unrecognised command '%s'.\n", input_buffer->buffer);
          continue;
        }
      }
    }

    Statement statement;
    switch (prepare_statement(input_buffer, &statement)) {
      case (PREPARE_SUCCESS): {
        break;
      }
      case (PREPARE_UNRECOGNISED_STATEMENT): {
        printf("Unrecognized keyword at start of '%s'.\n", input_buffer->buffer);
        continue;
      }
    }

    execute_statement(&statement);
    printf("Executed.\n");
  }
}
