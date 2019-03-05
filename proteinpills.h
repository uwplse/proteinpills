#ifndef __MAJORTOM
#define __MAJORTOM

void annotate_timeout(char *ty);
void annotate_message(char *ty);
void register_state_function(void (*)(void));

void int_field(char *field, int value);
void str_field(char *field, char *value);

#endif
