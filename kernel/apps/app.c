
#include "app.h"

static const char APP_NAME[] = "BLOXOS APP SYSTEM";
static const char APP_VERSION[] = "1.0.0";

static const char APP_DESCRIPTION[] =
    "BloxOS native application subsystem";

int blox_app_start(void)
{
    return 0;
}

const char* blox_app_name(void)
{
    return APP_NAME;
}

const char* blox_app_version(void)
{
    return APP_VERSION;
}

const char* blox_app_description(void)
{
    return APP_DESCRIPTION;
}
