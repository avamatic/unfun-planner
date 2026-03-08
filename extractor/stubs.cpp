// Minimal stubs for xNVSE common library symbols that we don't need
// but are referenced by compiled xNVSE sources.

#include <cstdio>
#include <cstdlib>

// IErrors.cpp — _AssertionFailed (yes, misspelled in xNVSE)
void _AssertionFailed(const char* file, unsigned long line, const char* desc)
{
	fprintf(stderr, "Assertion failed: %s (%s:%lu)\n", desc, file, line);
}

// IFileStream.cpp — IFileStream::MakeAllDirs
// Only called by IDebugLog::OpenRelative; we don't use that code path.
class IFileStream {
public:
	static void MakeAllDirs(const char* path);
};

void IFileStream::MakeAllDirs(const char* /* path */)
{
	// No-op stub — log directory creation not needed for plugin
}
