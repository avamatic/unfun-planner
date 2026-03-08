#include "extract.h"
#include "json_writer.h"

#include "nvse/GameAPI.h"
#include "nvse/GameData.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"

#include <fstream>
#include <ctime>
#include <cstdio>
#include <cctype>

#ifndef _MESSAGE
#define _MESSAGE(...)
#endif

// FNV Actor Value indices (verified against runtime extraction)
// SPECIAL stats: AV 5-11
static const int AV_STRENGTH     = 5;
static const int AV_PERCEPTION   = 6;
static const int AV_ENDURANCE    = 7;
static const int AV_CHARISMA     = 8;
static const int AV_INTELLIGENCE = 9;
static const int AV_AGILITY      = 10;
static const int AV_LUCK         = 11;

// Skills: AV 38-51 (39 = Big Guns, unused in FNV)
static const int AV_SKILLS_START = 38;
static const int AV_SKILLS_END   = 51;
static const int AV_BIG_GUNS     = 39; // unused in FNV

static const char* SPECIAL_ABBREVS[] = {"ST", "PE", "EN", "CH", "IN", "AG", "LK"};
static const char* SPECIAL_NAMES[]   = {"Strength", "Perception", "Endurance", "Charisma", "Intelligence", "Agility", "Luck"};

// Governing SPECIAL for each skill (AV index 38-51)
static int GetGoverningSpecialIndex(int skillAV) {
	switch (skillAV) {
		case 38: return 3; // Barter -> CH
		case 40: return 1; // Energy Weapons -> PE
		case 41: return 1; // Explosives -> PE
		case 42: return 1; // Lockpick -> PE
		case 43: return 4; // Medicine -> IN
		case 44: return 0; // Melee Weapons -> ST
		case 45: return 4; // Repair -> IN
		case 46: return 4; // Science -> IN
		case 47: return 5; // Guns -> AG
		case 48: return 5; // Sneak -> AG
		case 49: return 3; // Speech -> CH
		case 50: return 2; // Survival -> EN
		case 51: return 2; // Unarmed -> EN
		default: return -1;
	}
}

// Generate a PascalCase editor ID from a display name.
// "Energy Weapons" -> "EnergyWeapons", "Ain't Like That Now" -> "AintLikeThatNow"
static std::string GenerateEditorId(const char* name) {
	if (!name || !name[0]) return "";
	std::string result;
	bool capitalizeNext = true;
	for (const char* p = name; *p; p++) {
		if (*p == ' ' || *p == '-') {
			capitalizeNext = true;
		} else if (std::isalnum((unsigned char)*p)) {
			if (capitalizeNext) {
				result += (char)std::toupper((unsigned char)*p);
				capitalizeNext = false;
			} else {
				result += *p;
			}
		}
		// skip apostrophes, punctuation, etc.
	}
	return result;
}

// Forward declarations
static void WriteMetadata(JsonWriter& jw);
static void WriteSpecialAttributes(JsonWriter& jw);
static void WriteSkills(JsonWriter& jw);
static void WritePerks(JsonWriter& jw, bool traitsOnly);
static void WriteImplants(JsonWriter& jw);
static void WriteSkillBooks(JsonWriter& jw);
static void WriteSkillMagazines(JsonWriter& jw);
static void WriteLeveling(JsonWriter& jw);

bool ExtractAllBuildData(const char* outputPath)
{
	JsonWriter jw;

	jw.beginObject();

	jw.key("metadata");
	WriteMetadata(jw);

	jw.key("special_attributes");
	WriteSpecialAttributes(jw);

	jw.key("skills");
	WriteSkills(jw);

	jw.key("perks");
	WritePerks(jw, false);

	jw.key("traits");
	WritePerks(jw, true);

	jw.key("implants");
	WriteImplants(jw);

	jw.key("skill_books");
	WriteSkillBooks(jw);

	jw.key("skill_magazines");
	WriteSkillMagazines(jw);

	jw.key("leveling");
	WriteLeveling(jw);

	jw.endObject();

	std::ofstream outFile(outputPath);
	if (!outFile.is_open()) {
		_MESSAGE("BuildDataExtractor: Failed to open %s for writing", outputPath);
		return false;
	}

	outFile << jw.str();
	outFile.close();

	_MESSAGE("BuildDataExtractor: Successfully wrote %s", outputPath);
	return true;
}

// --- Metadata ---
static void WriteMetadata(JsonWriter& jw)
{
	jw.beginObject();

	jw.key("game_version");
	jw.valueString("1.4.0.525");

	jw.key("extractor_version");
	jw.valueString("1.1.0");

	jw.key("load_order");
	jw.beginArray();
	DataHandler* dh = DataHandler::Get();
	if (dh) {
		for (UInt32 i = 0; i < dh->modList.loadedModCount; i++) {
			ModInfo* mod = dh->modList.loadedMods[i];
			if (mod && mod->name[0]) {
				jw.valueString(mod->name);
			}
		}
	}
	jw.endArray();

	jw.key("extraction_date");
	time_t now = time(nullptr);
	char timeBuf[64];
	strftime(timeBuf, sizeof(timeBuf), "%Y-%m-%dT%H:%M:%SZ", gmtime(&now));
	jw.valueString(timeBuf);

	jw.endObject();
}

// --- SPECIAL Attributes ---
static void WriteSpecialAttributes(JsonWriter& jw)
{
	jw.beginArray();
	for (int i = 0; i < 7; i++) {
		jw.beginObject();
		jw.key("name");
		jw.valueString(SPECIAL_NAMES[i]);
		jw.key("abbreviation");
		jw.valueString(SPECIAL_ABBREVS[i]);
		jw.key("default_value");
		jw.valueInt(5);
		jw.key("min_value");
		jw.valueInt(1);
		jw.key("max_value");
		jw.valueInt(10);
		jw.endObject();
	}
	jw.endArray();
}

// --- Skills ---
static void WriteSkills(JsonWriter& jw)
{
	jw.beginArray();
	for (int i = AV_SKILLS_START; i <= AV_SKILLS_END; i++) {
		if (i == AV_BIG_GUNS) continue; // unused in FNV

		ActorValueInfo* av = GetActorValueInfo(i);
		if (!av) continue;

		const char* name = av->GetTheName();
		if (!name || !name[0]) continue;

		int govIdx = GetGoverningSpecialIndex(i);
		const char* govAbbrev = (govIdx >= 0) ? SPECIAL_ABBREVS[govIdx] : "??";

		jw.beginObject();

		jw.key("name");
		jw.valueString(name);

		jw.key("editor_id");
		// GetEditorID() returns empty at runtime; generate from name
		std::string edid = GenerateEditorId(name);
		jw.valueString(edid.c_str());

		jw.key("form_id");
		char formIdBuf[16];
		snprintf(formIdBuf, sizeof(formIdBuf), "%08X", av->refID);
		jw.valueString(formIdBuf);

		jw.key("governing_special");
		jw.valueString(govAbbrev);

		char formulaBuf[128];
		const char* govName = (govIdx >= 0) ? SPECIAL_NAMES[govIdx] : "unknown";
		snprintf(formulaBuf, sizeof(formulaBuf),
			"2 + (%s * 2) + ceil(luck / 2)", govName);
		jw.key("base_value_formula");
		jw.valueString(formulaBuf);

		jw.endObject();
	}
	jw.endArray();
}

// --- Perks & Traits ---
// In FNV, traits are BGSPerk forms with data.isTrait == 1
static void WritePerks(JsonWriter& jw, bool traitsOnly)
{
	DataHandler* dh = DataHandler::Get();
	if (!dh) { jw.beginArray(); jw.endArray(); return; }

	jw.beginArray();

	auto iter = dh->perkList.Begin();
	for (; !iter.End(); ++iter) {
		BGSPerk* perk = iter.Get();
		if (!perk) continue;

		bool isTrait = (perk->data.isTrait != 0);
		if (traitsOnly != isTrait) continue;

		if (!traitsOnly && !perk->data.isPlayable) continue;

		const char* name = perk->GetTheName();
		if (!name || !name[0]) continue;

		jw.beginObject();

		jw.key("name");
		jw.valueString(name);

		jw.key("editor_id");
		std::string edid = GenerateEditorId(name);
		jw.valueString(edid.c_str());

		jw.key("form_id");
		char formIdBuf[16];
		snprintf(formIdBuf, sizeof(formIdBuf), "%08X", perk->refID);
		jw.valueString(formIdBuf);

		jw.key("description");
		jw.valueString(name); // fallback — runtime descriptions need further work

		if (!traitsOnly) {
			jw.key("max_ranks");
			jw.valueInt(perk->data.numRanks);

			jw.key("level_requirement");
			jw.valueInt(perk->data.minLevel);

			jw.key("is_playable");
			jw.valueBool(perk->data.isPlayable != 0);

			// Prerequisites — condition parsing requires xNVSE-internal struct
			// layouts not fully exposed in public headers.
			// Level requirement is extracted above; detailed SPECIAL/skill/perk
			// prerequisites are available in the reference data JSON.
			jw.key("prerequisites");
			jw.beginObject();
			jw.key("special");
			jw.beginArray();
			jw.endArray();
			jw.key("skills");
			jw.beginArray();
			jw.endArray();
			jw.key("perks");
			jw.beginArray();
			jw.endArray();
			jw.endObject();
		}

		if (traitsOnly) {
			jw.key("effects");
			jw.beginArray();
			auto entryIter = perk->entries.Begin();
			for (; !entryIter.End(); ++entryIter) {
				BGSPerkEntry* entry = entryIter.Get();
				if (!entry) continue;
				jw.beginObject();
				jw.key("type");
				jw.valueString("other");
				jw.key("description");
				jw.valueString("");
				jw.endObject();
			}
			jw.endArray();
		}

		jw.endObject();
	}

	jw.endArray();
}

// --- Implants ---
// At runtime, editor IDs are stripped. Detect implants by name prefix "Implant"
// which matches vanilla FNV implant perks.
static void WriteImplants(JsonWriter& jw)
{
	DataHandler* dh = DataHandler::Get();
	jw.beginArray();

	if (dh) {
		auto iter = dh->perkList.Begin();
		for (; !iter.End(); ++iter) {
			BGSPerk* perk = iter.Get();
			if (!perk) continue;

			const char* name = perk->GetTheName();
			if (!name) continue;

			std::string nameStr(name);
			// Vanilla implant perks are named "Implant C-##" or similar
			// Also check for sub-dermal armor, monocyte breeder, etc.
			bool isImplant = (nameStr.find("Implant") == 0);
			if (!isImplant) {
				// Check for the non-SPECIAL implants by known names
				if (nameStr == "Sub-Dermal Armor" ||
				    nameStr == "Monocyte Breeder") {
					isImplant = true;
				}
			}
			if (!isImplant) continue;

			jw.beginObject();

			jw.key("name");
			jw.valueString(name);

			jw.key("editor_id");
			std::string edid = GenerateEditorId(name);
			jw.valueString(edid.c_str());

			jw.key("form_id");
			char formIdBuf[16];
			snprintf(formIdBuf, sizeof(formIdBuf), "%08X", perk->refID);
			jw.valueString(formIdBuf);

			jw.key("effects");
			jw.beginArray();
			jw.beginObject();

			// Determine effect type from name
			jw.key("type");
			if (nameStr.find("Implant") == 0 &&
			    nameStr != "Sub-Dermal Armor" &&
			    nameStr != "Monocyte Breeder" &&
			    nameStr.find("GRX") == std::string::npos) {
				jw.valueString("special_modifier");
			} else {
				jw.valueString("derived_stat");
			}

			// Determine target from name
			jw.key("target");
			if (nameStr.find("Strength") != std::string::npos ||
			    nameStr.find("STR") != std::string::npos ||
			    nameStr.find("C-13") != std::string::npos)
				jw.valueString("ST");
			else if (nameStr.find("Perception") != std::string::npos ||
			         nameStr.find("PER") != std::string::npos ||
			         nameStr.find("C-14") != std::string::npos)
				jw.valueString("PE");
			else if (nameStr.find("Endurance") != std::string::npos ||
			         nameStr.find("END") != std::string::npos ||
			         nameStr.find("C-15") != std::string::npos)
				jw.valueString("EN");
			else if (nameStr.find("Charisma") != std::string::npos ||
			         nameStr.find("CHA") != std::string::npos ||
			         nameStr.find("C-16") != std::string::npos)
				jw.valueString("CH");
			else if (nameStr.find("Intelligence") != std::string::npos ||
			         nameStr.find("INT") != std::string::npos ||
			         nameStr.find("C-17") != std::string::npos)
				jw.valueString("IN");
			else if (nameStr.find("Agility") != std::string::npos ||
			         nameStr.find("AGI") != std::string::npos ||
			         nameStr.find("C-18") != std::string::npos)
				jw.valueString("AG");
			else if (nameStr.find("Luck") != std::string::npos ||
			         nameStr.find("LCK") != std::string::npos ||
			         nameStr.find("C-19") != std::string::npos)
				jw.valueString("LK");
			else if (nameStr == "Sub-Dermal Armor")
				jw.valueString("DT");
			else if (nameStr == "Monocyte Breeder")
				jw.valueString("HP_REGEN");
			else
				jw.valueString(name);

			jw.key("magnitude");
			jw.valueInt(1);

			jw.key("description");
			jw.valueString(name);

			jw.endObject();
			jw.endArray(); // effects

			jw.endObject();
		}
	}

	jw.endArray();
}

// --- Skill Books ---
// TESObjectBOOK is only forward-declared in xNVSE headers (not fully defined).
// Skill book data is available in the reference JSON instead.
static void WriteSkillBooks(JsonWriter& jw)
{
	jw.beginArray();
	jw.endArray();
}

// --- Skill Magazines ---
static void WriteSkillMagazines(JsonWriter& jw)
{
	jw.beginArray();
	jw.endArray();
}

// --- Leveling ---
static void WriteLeveling(JsonWriter& jw)
{
	jw.beginObject();

	jw.key("max_level");
	jw.valueInt(30);

	jw.key("perk_interval");
	jw.valueInt(2);

	jw.key("skill_points_per_level");
	jw.beginObject();
	jw.key("base");
	jw.valueInt(10);
	jw.key("intelligence_multiplier");
	jw.valueFloat(0.5f);
	jw.endObject();

	jw.key("tag_bonus");
	jw.valueInt(15);

	jw.key("skill_cap");
	jw.valueInt(100);

	jw.key("special_points_at_creation");
	jw.valueInt(40);

	jw.endObject();
}
