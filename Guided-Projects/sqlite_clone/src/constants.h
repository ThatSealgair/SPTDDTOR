#define EXIT_COMMAND ".exit"
#define EXIT_COMMAND_SIZE 5
#define META_COMMAND '.'

#define USER_INSERT "insert"
#define USER_INSERT_SIZE 6
#define USER_SELECT "select"
#define USER_SELECT_SIZE 6

typedef enum {
  META_COMMAND_SUCCESS,
  META_COMMAND_UNRECOGNISED_COMMAND,
} MetaCommandResult;

typedef enum {
  PREPARE_SUCCESS,
  PREPARE_UNRECOGNISED_STATEMENT,
} PrepareResult;

typedef enum {
  STATEMENT_INSERT,
  STATEMENT_SELECT,
} StatementType;
