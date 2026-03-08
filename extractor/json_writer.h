#pragma once

#include <string>
#include <vector>
#include <sstream>

// Minimal JSON writer — no external dependencies needed.
class JsonWriter {
public:
	JsonWriter() : m_indent(0), m_needsComma(false) {}

	void beginObject() {
		maybeComma();
		m_ss << "{\n";
		m_indent++;
		m_needsComma = false;
	}

	void endObject() {
		m_ss << "\n";
		m_indent--;
		writeIndent();
		m_ss << "}";
		m_needsComma = true;
	}

	void beginArray() {
		maybeComma();
		m_ss << "[\n";
		m_indent++;
		m_needsComma = false;
	}

	void endArray() {
		m_ss << "\n";
		m_indent--;
		writeIndent();
		m_ss << "]";
		m_needsComma = true;
	}

	void key(const char* k) {
		maybeComma();
		m_ss << "\"" << escape(k) << "\": ";
		m_needsComma = false;
	}

	void valueString(const char* v) {
		maybeComma();
		m_ss << "\"" << escape(v ? v : "") << "\"";
		m_needsComma = true;
	}

	void valueInt(int v) {
		maybeComma();
		m_ss << v;
		m_needsComma = true;
	}

	void valueFloat(float v) {
		maybeComma();
		m_ss << v;
		m_needsComma = true;
	}

	void valueBool(bool v) {
		maybeComma();
		m_ss << (v ? "true" : "false");
		m_needsComma = true;
	}

	void valueNull() {
		maybeComma();
		m_ss << "null";
		m_needsComma = true;
	}

	std::string str() const { return m_ss.str(); }

private:
	std::stringstream m_ss;
	int m_indent;
	bool m_needsComma;

	void writeIndent() {
		for (int i = 0; i < m_indent; i++)
			m_ss << "  ";
	}

	void maybeComma() {
		if (m_needsComma) {
			m_ss << ",\n";
		}
		writeIndent();
	}

	static std::string escape(const char* s) {
		std::string result;
		for (const char* p = s; *p; p++) {
			switch (*p) {
				case '"':  result += "\\\""; break;
				case '\\': result += "\\\\"; break;
				case '\n': result += "\\n"; break;
				case '\r': result += "\\r"; break;
				case '\t': result += "\\t"; break;
				default:   result += *p; break;
			}
		}
		return result;
	}
};
