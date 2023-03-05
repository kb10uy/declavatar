#include <stdio.h>
#include <stdint.h>
#include <stdlib.h>
#include "declavatar.h"

#define AVATAR_KDL "version \"1.0.0\""

void print_errors(Declavatar *da);

int main(void) {
    Declavatar *da = DeclavatarInit();

    DeclavatarReset(da);
    if (DeclavatarCompile(da, AVATAR_KDL) != DA_STATUS_SUCCESS) {
        print_errors(da);
    } else {
        uint32_t json_length;
        char *json_source;
        char *json_copied;

        DeclavatarGetAvatarJson(da, &json_source, &json_length);
        json_copied = (char *) calloc((size_t) json_length + 1, sizeof(char));
        memcpy(json_copied, json_source, json_length);
        printf("%s\n", json_copied);
        free(json_copied);
    }

    DeclavatarFree(da);
    return 0;
}

void print_errors(Declavatar *da) {
    uint32_t errors = 0;

    DeclavatarGetErrorsCount(da, &errors);
    for (uint32_t i = 0; i < errors; ++i) {
        uint32_t error_length;
        DeclavatarErrorKind kind;
        char *error_source;
        char *error_copied;

        DeclavatarGetError(da, i, &kind, &error_source, &error_length);
        error_copied = (char *) calloc((size_t) error_length + 1, sizeof(char));
        memcpy(error_copied, error_source, error_length);
        printf("#%02d: [%d] %s\n", i, kind, error_copied);
        free(error_copied);
    }

    printf("\n");
}
