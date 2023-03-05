#pragma once

#include <stdint.h>

typedef struct Declavatar;
typedef uint32_t DeclavatarStatusCode;
typedef uint32_t DeclavatarErrorKind;

enum {
    DA_STATUS_SUCCESS = 0,
    DA_STATUS_UTF8_ERROR = 1,
    DA_STATUS_COMPILE_ERROR = 2,
    DA_STATUS_ALREADY_IN_USE = 3,
    DA_STATUS_NOT_COMPILED = 4,
    DA_STATUS_INVALID_POINTER = 128,
};

enum {
    DA_ERROR_COMPILER_ERROR = 0,
    DA_ERROR_SYNTAX_ERROR = 1,
    DA_ERROR_SEMANTIC_ERROR = 2,
    DA_ERROR_SEMANTIC_INFO = 3,
};

Declavatar *DeclavatarInit(void);
DeclavatarStatusCode DeclavatarFree(Declavatar *da);
DeclavatarStatusCode DeclavatarReset(Declavatar *da);
DeclavatarStatusCode DeclavatarCompile(Declavatar *da, const char *source);
DeclavatarStatusCode DeclavatarGetAvatarJson(Declavatar *da, const char **json, uint32_t *jsonLength);
DeclavatarStatusCode DeclavatarGetErrorsCount(Declavatar *da, uint32_t *errorCount);
DeclavatarStatusCode DeclavatarGetError(Declavatar *da, uint32_t index, DeclavatarStatusCode *errorKind, const char **json, uint32_t *errorLength);
