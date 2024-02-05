#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * Declavatar definition file format..
 */
enum DeclavatarFormat {
  /**
   * S-expression.
   */
  DECLAVATAR_FORMAT_SEXPR = 1,
  /**
   * Lua.
   */
  DECLAVATAR_FORMAT_LUA = 2,
};
typedef uint32_t DeclavatarFormat;

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

typedef struct CompiledState CompiledState;

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

/**
 * Defines a localization for given state.
 *
 * # Safety
 * Given pointers must be valid.
 * `key`, `value` does not have to NUL-terminated.
 */
DeclavatarStatus declavatar_define_localization(struct DeclavatarState *da,
                                                const char *key,
                                                uint32_t key_len,
                                                const char *value,
                                                uint32_t value_len);

/**
 * Registers Arbitrary Attachment (arbittach) definition.
 *
 * # Safety
 * Given pointers must be valid.
 * `definition` does not have to NUL-terminated.
 */
DeclavatarStatus declavatar_register_arbittach(struct DeclavatarState *da,
                                               const char *definition,
                                               uint32_t definition_len);

/**
 * Compiles definition with format.
 *
 * # Safety
 * Given pointers must be valid.
 * `source` does not have to NUL-terminated.
 */
DeclavatarStatus declavatar_compile(struct DeclavatarState *da,
                                    struct CompiledState **compiled_state,
                                    const char *source,
                                    uint32_t source_len,
                                    DeclavatarFormat format_kind);

/**
 * Frees compiled result.
 *
 * # Safety
 * Given pointer must be valid.
 */
DeclavatarStatus declavatar_free_compiled(struct CompiledState *compiled_state);
