#ifndef __MAJORTOM
#define __MAJORTOM

#ifdef __cplusplus
extern "C" {
#endif
void annotate_message(char *ty);
void annotate_timeout(char *ty);
void register_state_function(void (*)(void));

void int_field(char *field, int value);
void str_field(char *field, char *value);

#ifdef __cplusplus
}
#endif

#endif
