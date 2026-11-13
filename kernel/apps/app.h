
#ifndef BLOXOS_APP_H
#define BLOXOS_APP_H

#ifdef __cplusplus
extern "C" {
#endif

int blox_app_start(void);

const char* blox_app_name(void);
const char* blox_app_version(void);
const char* blox_app_description(void);

#ifdef __cplusplus
}
#endif

#endif
