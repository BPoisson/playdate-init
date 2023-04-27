#include "pd_api.h"

static int update(void* update);

#ifdef _WINDLL
__declspec(dllexport)
#endif
int eventHandler(PlaydateAPI* pd, PDSystemEvent event, uint32_t arg)
{
	if (event == kEventInit)
	{
		pd->system->setUpdateCallback(update, NULL);
	}
	return 0;
}

static int update(void* update)
{
	return 1;
}

