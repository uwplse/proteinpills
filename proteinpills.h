#ifndef __MAJORTOM
#define __MAJORTOM

#ifdef __cplusplus
extern "C" {
#endif
void annotate_message(const char *ty);
void annotate_timeout(const char *ty);
void register_state_function(void (*)(void));

void int_field(const char *field, int value);
void str_field(const char *field, const char *value);
void protobuf_field(const char *field, const char *value, size_t value_len);

#ifdef __cplusplus
}
#endif

#endif
