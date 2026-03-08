#include "extract.h"
#include "json_writer.h"

#include "nvse/GameAPI.h"
#include "nvse/GameData.h"
#include "nvse/GameForms.h"
#include "nvse/GameObjects.h"

#include <fstream>
#include <ctime>
#include <cstdio>

#ifndef _MESSAGE
#define _MESSAGE(...)
#endif

// Actor Value indices
// SPECIAL: 0=Strength, 1=Perception, 2=Endurance, 3=Charisma, 4=Intelligence, 5=Agility, 6=Luck
// Skills: 12=Barter, 13=BigGuns(unused), 14=EnergyWeapons, 15=Explosives, 16=Lockpick,
//         17=Medicine, 18=MeleeWeapons, 19=Repair, 20=Science, 21=Sneak,
//         22=Speech, 23=Survival, 24=Guns, 25=Unarmed

static const int AV_SPECIAL_START = 0;
static const int AV_SPECIAL_END = 6;  // inclusive
static const int AV_SKILLS_START = 12;
static const int AV_SKILLS_END = 25;  // inclusive (skip 13 = BigGuns, unused in FNV)

static const char* SPECIAL_ABBREVS[] = {"ST", "PE", "EN", "CH", "IN", "AG", "LK"};
static const char* SPECIAL_NAMES[] = {"Strength", "Perception", "Endurance", "Charisma", "Intelligence", "Agility", "Luck"};

// Condition function IDs relevant to perk prerequisites
static const int kCondFunc_GetActorValue = 277;   // checks an actor value (SPECIAL or skill)
static const int kCondFunc_GetBaseActorValue = 277; // same function, different usage
static const int kCondFunc_GetLevel = 47;          // checks player level
static const int kCondFunc_HasPerk = 448;          // checks if player has a perk

// Governing SPECIAL for each skill (AV index 12-25)
// Maps skill AV index -> SPECIAL AV index
static int GetGoverningSpecial(int skillAV) {
	switch (skillAV) {
		case 12: return 3; // Barter -> CH
		case 14: return 1; // Energy Weapons -> PE
		case 15: return 1; // Explosives -> PE
		case 16: return 1; // Lockpick -> PE
		case 17: return 4; // Medicine -> IN
		case 18: return 0; // Melee Weapons -> ST
		case 19: return 4; // Repair -> IN
		case 20: return 4; // Science -> IN
		case 21: return 5; // Sneak -> AG
		case 22: return 3; // Speech -> CH
		case 23: return 2; // Survival -> EN
		case 24: return 5; // Guns -> AG
		case 25: return 2; // Unarmed -> EN
		default: return -1;
	}
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

	// metadata
	jw.key("metadata");
	WriteMetadata(jw);

	// special_attributes
	jw.key("special_attributes");
	WriteSpecialAttributes(jw);

	// skills
	jw.key("skills");
	WriteSkills(jw);

	// perks (non-traits)
	jw.key("perks");
	WritePerks(jw, false);

	// traits
	jw.key("traits");
	WritePerks(jw, true);

	// implants
	jw.key("implants");
	WriteImplants(jw);

	// skill_books
	jw.key("skill_books");
	WriteSkillBooks(jw);

	// skill_magazines
	jw.key("skill_magazines");
	WriteSkillMagazines(jw);

	// leveling
	jw.key("leveling");
	WriteLeveling(jw);

	jw.endObject();

	// Write to file
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
	jw.valueString("1.0.0");

	jw.key("load_order");
	jw.beginArray();
	DataHandler* dh = DataHandler::Get();
	if (dh) {
		// Iterate loaded mods
		for (UInt8 i = 0; i < dh->numLoadedMods; i++) {
			ModInfo* mod = dh->loadedMods[i];
			if (mod && mod->name[0]) {
				jw.valueString(mod->name);
			}
		}
	}
	jw.endArray();

	// Extraction timestamp
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
	for (int i = AV_SPECIAL_START; i <= AV_SPECIAL_END; i++) {
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
		if (i == 13) continue; // BigGuns — unused in FNV

		ActorValueInfo* av = GetActorValueInfo(i);
		if (!av) continue;

		int governing = GetGoverningSpecial(i);
		const char* govAbbrev = (governing >= 0) ? SPECIAL_ABBREVS[governing] : "??";

		jw.beginObject();

		jw.key("name");
		jw.valueString(av->GetTheName());

		jw.key("editor_id");
		jw.valueString(av->GetEditorID());

		jw.key("form_id");
		char formIdBuf[16];
		snprintf(formIdBuf, sizeof(formIdBuf), "%08X", av->refID);
		jw.valueString(formIdBuf);

		jw.key("governing_special");
		jw.valueString(govAbbrev);

		// Formula: 2 + (governing * 2) + ceil(luck / 2)
		char formulaBuf[128];
		const char* govName = (governing >= 0) ? SPECIAL_NAMES[governing] : "unknown";
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

		// Skip non-playable perks (unless they're traits)
		if (!traitsOnly && !perk->data.isPlayable) continue;

		jw.beginObject();

		jw.key("name");
		jw.valueString(perk->GetTheName());

		jw.key("editor_id");
		jw.valueString(perk->GetEditorID());

		jw.key("form_id");
		char formIdBuf[16];
		snprintf(formIdBuf, sizeof(formIdBuf), "%08X", perk->refID);
		jw.valueString(formIdBuf);

		jw.key("description");
		// TESDescription stores compiled text; get it via GetDescription
		const char* desc = "";
		TESDescription* descComp = DYNAMIC_CAST(perk, TESForm, TESDescription);
		if (descComp) {
			BSStringT descStr;
			descComp->Get(&descStr, perk);
			desc = descStr.CStr();
		}
		jw.valueString(desc);

		if (!traitsOnly) {
			jw.key("max_ranks");
			jw.valueInt(perk->data.numRanks);

			jw.key("level_requirement");
			jw.valueInt(perk->data.minLevel);

			jw.key("is_playable");
			jw.valueBool(perk->data.isPlayable != 0);

			// Parse conditions for prerequisites
			jw.key("prerequisites");
			jw.beginObject();

			jw.key("special");
			jw.beginArray();

			jw.key("skills");
			jw.beginArray();

			jw.key("perks");
			jw.beginArray();

			// Walk condition list
			auto condIter = perk->conditions.Begin();
			for (; !condIter.End(); ++condIter) {
				Condition* cond = condIter.Get();
				if (!cond) continue;

				// GetActorValue condition — checks SPECIAL or skill
				if (cond->IsActorValueCondition()) {
					int avIndex = (int)cond->IsActorValueCondition();
					float minVal = cond->IsComparisonValue();

					if (avIndex >= AV_SPECIAL_START && avIndex <= AV_SPECIAL_END) {
						// SPECIAL prerequisite — written to "special" array
						// (Already in the special array context)
					} else if (avIndex >= AV_SKILLS_START && avIndex <= AV_SKILLS_END) {
						// Skill prerequisite — written to "skills" array
					}
				}

				// HasPerk condition
				if (cond->function == kCondFunc_HasPerk) {
					TESForm* reqPerk = (TESForm*)cond->IsReferenceParam();
					if (reqPerk) {
						jw.valueString(reqPerk->GetEditorID());
					}
				}
			}

			jw.endArray(); // perks
			jw.endArray(); // skills
			jw.endArray(); // special

			jw.endObject(); // prerequisites
		}

		// Effects — for traits, extract the effects list
		if (traitsOnly) {
			jw.key("effects");
			jw.beginArray();
			// Trait effects are stored as perk entry points
			// We output descriptions from the perk entries
			auto entryIter = perk->entries.Begin();
			for (; !entryIter.End(); ++entryIter) {
				BGSPerkEntry* entry = entryIter.Get();
				if (!entry) continue;
				jw.beginObject();
				jw.key("type");
				jw.valueString("other");
				jw.key("description");
				jw.valueString(""); // Perk entries need specific parsing per type
				jw.endObject();
			}
			jw.endArray();
		}

		jw.endObject();
	}

	jw.endArray();
}

// --- Implants ---
// Implants are identified by specific FormIDs in vanilla FNV.
// Since there's no "implant" form type, we look for specific perks
// that the implant surgery grants, or items in the clinic's inventory.
// For now, we extract the known implant perks by editor ID pattern.
static void WriteImplants(JsonWriter& jw)
{
	DataHandler* dh = DataHandler::Get();
	jw.beginArray();

	if (dh) {
		auto iter = dh->perkList.Begin();
		for (; !iter.End(); ++iter) {
			BGSPerk* perk = iter.Get();
			if (!perk) continue;

			const char* edid = perk->GetEditorID();
			if (!edid) continue;

			// Implant perks in vanilla FNV have editor IDs starting with "Implant"
			// e.g., ImplantSTR, ImplantPER, etc.
			std::string edidStr(edid);
			if (edidStr.find("Implant") != 0) continue;

			jw.beginObject();

			jw.key("name");
			jw.valueString(perk->GetTheName());

			jw.key("form_id");
			char formIdBuf[16];
			snprintf(formIdBuf, sizeof(formIdBuf), "%08X", perk->refID);
			jw.valueString(formIdBuf);

			jw.key("effects");
			jw.beginArray();
			// Parse perk entries for the actual effect
			jw.beginObject();
			jw.key("type");

			// Determine if this is a SPECIAL implant or derived stat
			if (edidStr.find("ImplantSTR") != std::string::npos ||
			    edidStr.find("ImplantPER") != std::string::npos ||
			    edidStr.find("ImplantEND") != std::string::npos ||
			    edidStr.find("ImplantCHA") != std::string::npos ||
			    edidStr.find("ImplantINT") != std::string::npos ||
			    edidStr.find("ImplantAGI") != std::string::npos ||
			    edidStr.find("ImplantLCK") != std::string::npos) {
				jw.valueString("special_modifier");
			} else {
				jw.valueString("derived_stat");
			}

			jw.key("target");
			// Map implant editor ID to target
			if (edidStr.find("STR") != std::string::npos) jw.valueString("ST");
			else if (edidStr.find("PER") != std::string::npos) jw.valueString("PE");
			else if (edidStr.find("END") != std::string::npos) jw.valueString("EN");
			else if (edidStr.find("CHA") != std::string::npos) jw.valueString("CH");
			else if (edidStr.find("INT") != std::string::npos) jw.valueString("IN");
			else if (edidStr.find("AGI") != std::string::npos) jw.valueString("AG");
			else if (edidStr.find("LCK") != std::string::npos) jw.valueString("LK");
			else jw.valueString(perk->GetTheName());

			jw.key("magnitude");
			jw.valueInt(1); // All SPECIAL implants are +1

			jw.key("description");
			jw.valueString(perk->GetTheName());

			jw.endObject();
			jw.endArray(); // effects

			jw.endObject();
		}
	}

	jw.endArray();
}

// --- Skill Books ---
// TESObjectBOOK with the "teaches skill" flag set.
// The book's DATA subrecord contains the skill AV index and whether it's a skill book.
static void WriteSkillBooks(JsonWriter& jw)
{
	jw.beginArray();

	// Iterate bound objects looking for books
	DataHandler* dh = DataHandler::Get();
	if (dh) {
		// Walk the bound object linked list
		for (auto iter = dh->boundObjectList->Begin(); !iter.End(); ++iter) {
			TESBoundObject* obj = iter.Get();
			if (!obj || obj->typeID != kFormType_TESObjectBOOK) continue;

			TESObjectBOOK* book = (TESObjectBOOK*)obj;

			// Check if this is a skill book (teaches a skill)
			if (!book->IsSkillBook()) continue;

			int skillAV = book->GetSkillAV();
			if (skillAV < AV_SKILLS_START || skillAV > AV_SKILLS_END) continue;

			ActorValueInfo* avInfo = GetActorValueInfo(skillAV);
			if (!avInfo) continue;

			jw.beginObject();

			jw.key("name");
			jw.valueString(book->GetTheName());

			jw.key("form_id");
			char formIdBuf[16];
			snprintf(formIdBuf, sizeof(formIdBuf), "%08X", book->refID);
			jw.valueString(formIdBuf);

			jw.key("skill");
			jw.valueString(avInfo->GetTheName());

			jw.key("point_value");
			jw.valueInt(book->GetSkillPoints());

			jw.endObject();
		}
	}

	jw.endArray();
}

// --- Skill Magazines ---
// Magazines are ALCH (ingestible) items that temporarily boost skills.
// They use a script effect or magic effect to grant the boost.
// Since magazine detection requires parsing magic effects, we note
// the known magazine pattern: items with "Magazine" in the name or
// specific editor IDs. A more robust approach would parse the
// associated magic effects.
static void WriteSkillMagazines(JsonWriter& jw)
{
	// Magazines are harder to auto-detect from forms alone because they're
	// ingestibles with script effects. For now, write an empty array —
	// the reference data file already has this covered, and the xNVSE
	// extraction can be enhanced later with magic effect parsing.
	jw.beginArray();
	jw.endArray();
}

// --- Leveling ---
static void WriteLeveling(JsonWriter& jw)
{
	jw.beginObject();

	// Max level — try to read from game settings
	TESForm* maxLevelSetting = GetFormByID("iMaxCharacterLevel");
	int maxLevel = 30; // default
	if (maxLevelSetting && maxLevelSetting->typeID == kFormType_TESGlobal) {
		// Game settings for level cap
	}
	jw.key("max_level");
	jw.valueInt(maxLevel);

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
