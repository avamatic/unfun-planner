#include "nvse/PluginAPI.h"
#include "nvse/CommandTable.h"
#include "nvse/GameAPI.h"
#include "nvse/ParamInfos.h"

#include "extract.h"

// Version constant — FNV 1.4.0.525 (final patch)
#ifndef RUNTIME_VERSION_1_4_0_525
#define RUNTIME_VERSION_1_4_0_525 0x04000211
#endif

// Logging stub if _MESSAGE is not available
#ifndef _MESSAGE
#define _MESSAGE(...)
#endif

// Packed NVSE version if not defined
#ifndef PACKED_NVSE_VERSION
#define PACKED_NVSE_VERSION 0
#endif

static PluginHandle g_pluginHandle = kPluginHandle_Invalid;
static const NVSEInterface* g_nvseInterface = nullptr;
static NVSEMessagingInterface* g_messagingInterface = nullptr;

#define PLUGIN_VERSION 1

// Optional: path parameter for output file
static ParamInfo kParams_OneOptionalString[] = {
	{"string", kParamType_String, 1}, // optional
};

#if RUNTIME

bool Cmd_ExtractBuildData_Execute(COMMAND_ARGS)
{
	*result = 0;

	char outputPath[512] = {0};
	if (!ExtractArgs(EXTRACT_ARGS, &outputPath))
		outputPath[0] = '\0';

	// Default output path
	const char* path = outputPath[0] ? outputPath : "Data\\build_data.json";

	Console_Print("BuildDataExtractor: Extracting game data...");

	bool success = ExtractAllBuildData(path);

	if (success) {
		Console_Print("BuildDataExtractor: Data written to %s", path);
		*result = 1;
	} else {
		Console_Print("BuildDataExtractor: Extraction failed!");
	}

	return true;
}

DEFINE_COMMAND_PLUGIN(ExtractBuildData, "Extract character build data to JSON", false, kParams_OneOptionalString)

#endif // RUNTIME

extern "C" {

bool NVSEPlugin_Query(const NVSEInterface* nvse, PluginInfo* info)
{
	info->infoVersion = PluginInfo::kInfoVersion;
	info->name = "BuildDataExtractor";
	info->version = PLUGIN_VERSION;

	if (nvse->isEditor)
		return false;

	return true;
}

bool NVSEPlugin_Load(NVSEInterface* nvse)
{
	g_pluginHandle = nvse->GetPluginHandle();
	g_nvseInterface = nvse;

	g_messagingInterface = (NVSEMessagingInterface*)nvse->QueryInterface(kInterface_Messaging);

	// Register the console command
	nvse->SetOpcodeBase(0x3A00);
	REG_CMD(ExtractBuildData);

	return true;
}

} // extern "C"
