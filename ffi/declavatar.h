#pragma once

#include <stdint.h>

typedef struct Declavatar;
typedef uint32_t DeclavatarStatusCode;
typedef uint32_t DeclavatarErrorKind;
typedef uint32_t DeclavatarFormat;

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

enum {
    DA_FORMAT_SEXPR = 1,
    DA_FORMAT_LUA = 2,
};

Declavatar *DeclavatarInit(void);
DeclavatarStatusCode DeclavatarFree(Declavatar *da);
DeclavatarStatusCode DeclavatarReset(Declavatar *da);
DeclavatarStatusCode DeclavatarAddLibraryPath(Declavatar *da, const char *path, uint32_t pathLength);
DeclavatarStatusCode DeclavatarDefineSymbol(Declavatar *da, const char *symbol, uint32_t symbolLength);
DeclavatarStatusCode DeclavatarDefineLocalization(Declavatar *da, const char *key, uint32_t keyLength, const char *value, uint32_t valueLength);
DeclavatarStatusCode DeclavatarCompile(Declavatar *da, const char *source, uint32_t sourceLength, DeclavatarFormat format);
DeclavatarStatusCode DeclavatarGetAvatarJson(Declavatar *da, const char **json, uint32_t *jsonLength);
DeclavatarStatusCode DeclavatarGetLogsCount(Declavatar *da, uint32_t *errorCount);
DeclavatarStatusCode DeclavatarGetLogJson(Declavatar *da, uint32_t index, const char **json, uint32_t *errorLength);
DeclavatarStatusCode DeclavatarGetI18n(const char *i18nKey, uint32_t i18nKeyLength, const char **i18nJson, uint32_t *i18nJsonLength);
