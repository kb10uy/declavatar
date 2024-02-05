#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Declavatar status code.
 */
enum DeclavatarStatus {
  /**
   * Operation succeeded.
   */
  DECLAVATAR_STATUS_SUCCESS = 0,
  /**
   * Given string has invalid UTF-8 sequence.
   */
  DECLAVATAR_STATUS_UTF8_ERROR = 1,
  /**
   * Internal JSON serialization error.
   */
  DECLAVATAR_STATUS_JSON_ERROR = 2,
  /**
   * Error occurred in compilation.
   */
  DECLAVATAR_STATUS_COMPILE_ERROR = 3,
  /**
   * Given pointer was invalid.
   */
  DECLAVATAR_STATUS_INVALID_POINTER = 128,
};
typedef uint32_t DeclavatarStatus;

typedef struct DeclavatarState DeclavatarState;

/**
 * Initializes declavatar compiler state.
 */
struct DeclavatarState *declavatar_init(void);

/**
 * Frees declavatar compiler state.
 */
DeclavatarStatus declavatar_free(struct DeclavatarState *da);

/**
 * Clears defined symbols/localizations/arbittach definitions.
 *
 * # Safety
 * Given pointer `da` must be valid.
 */
DeclavatarStatus declavatar_clear(struct DeclavatarState *da);

/**
 * Clears defined symbols/localizations/arbittach definitions.
 *
 * # Safety
 * Given pointers must be valid.
 * `path` does not have to NUL-terminated.
 */
DeclavatarStatus declavatar_add_library_path(struct DeclavatarState *da,
                                             const char *path,
                                             uint32_t path_len);

/**
 * Defines a symbol for given state.
 *
 * # Safety
 * Given pointers must be valid.
 * `symbol` does not have to NUL-terminated.
 */
DeclavatarStatus declavatar_define_symbol(struct DeclavatarState *da,
                                          const char *symbol,
                                          uint32_t symbol_len);
