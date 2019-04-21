#include "common/hercules.h"
#include "common/nullpo.h"
#include "map/map.h"
#include "map/script.h"

#include <stdio.h>
#ifdef WIN32
#include <windows.h>
#else
#include <stdlib.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>
#endif

#include "common/HPMDataCheck.h"

HPExport struct hplugin_info pinfo = {
	"vscode_syntax",    // Plugin name
	SERVER_TYPE_MAP,     // Which server types this plugin works with?
	"1.0",               // Plugin version
	HPM_VERSION,         // HPM Version (don't change, macro is automatically updated)
};

// Guessed values
#define BUFFER_SIZE 100000
#define MAX_COMMENT_LEN 100
#define MAX_CONST_NAME_LEN 50
#define CASE_SENSITIVE

#ifdef CASE_SENSITIVE
#define MATCH_START "\\\\b"
#else
#define MATCH_START "(?i)\\b"
#endif
#define MATCH_END "\\\\b"

FILE *out;

char _comment[MAX_COMMENT_LEN];
char _buffer[BUFFER_SIZE];
int _cmd_count;
bool deprecated_cmd;
int const_count;

bool torun = false;

void vscodesyntax_clear_buffers(void)
{
	strcpy(_comment, "");
	strcpy(_buffer, "");
}

// ===========================
// Constants
// ===========================
void vscodesyntax_flush_constgroup(void)
{
	nullpo_retv(out);

	if (strnlen(_buffer, 3) == 0)
		return;

	if (const_count > 0) {
		fprintf(out, ",\n");
	}
	const_count++;

	const_count++;
	fprintf(out,
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>%s</string>\n"
		// "					<key>name</key>\n"
		// "					<string>constant.language</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b(%s)\\b</string>\n"
		// "				</dict>\n",
		"				{\n"
		"					\"comment\": \"%s\",\n"
		"					\"name\": \"constant.language\",\n"
		"					\"match\": \""MATCH_START"(%s)"MATCH_END"\"\n"
		"				}",
		_comment, _buffer
	);

	vscodesyntax_clear_buffers();
}

void vscodesyntax_constdb_comment(const char *comment)
{
	nullpo_retv(out);

	vscodesyntax_flush_constgroup();

	if (comment != NULL)
		strncpy(_comment, comment, MAX_COMMENT_LEN-1);
}

void vscodesyntax_script_set_constant(const char *name, int value, bool is_parameter, bool is_deprecated)
{
	nullpo_retv(out);

	if (name != NULL) {
		if (strnlen(_buffer, 5) >= 2) {
			char aux[500];
			sprintf(aux, "|%s", name);
			strcat(_buffer, aux);
		} else {
			sprintf(_buffer, "%s", name);
		}
	}
}

void vscodesyntax_constdb(void)
{
	void(*script_set_constant) (const char *name, int value, bool is_parameter, bool is_deprecated) = NULL;
	void(*script_constdb_comment) (const char *comment) = NULL;

	const_count = 0;

	// Save links
	script_set_constant = script->set_constant;
	script_constdb_comment = script->constdb_comment;

	// Change links
	script->set_constant = vscodesyntax_script_set_constant;
	script->constdb_comment = vscodesyntax_constdb_comment;

	// Read constants
	script->read_constdb(false);
	script->load_parameters();
	script->hardcoded_constants();

	// Restore original links
	script->set_constant = script_set_constant;
	script->constdb_comment = script_constdb_comment;
}

// ===========================
// Maps
// ===========================
/// Cloned from mapindex_init / Haru
void vscodesyntax_mapdb(void)
{
	FILE *mfp;
	char line[1024];
	int index;
	char map_name[MAP_NAME_LENGTH];
	char *mapindex_cfgfile = "db/map_index.txt";
	int count = 0;

	nullpo_retv(out);

	fprintf(out, 
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Maps</string>\n"
		// "					<key>name</key>\n"
		// "					<string>constant.language.maps</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b("
		"				{\n"
		"					\"comment\": \"Maps\",\n"
		"					\"name\": \"constant.language.maps\",\n"
		"					\"match\": \""MATCH_START"("
	);
	
	if ((mfp = fopen(mapindex_cfgfile, "r")) == NULL) {
		ShowFatalError("Unable to read mapindex config file %s!\n", mapindex_cfgfile);
		return;
	}

	while (fgets(line, sizeof(line), mfp)) {
		if (line[0] == '/' && line[1] == '/')
			continue;

		switch (sscanf(line, "%11s\t%d", map_name, &index)) {
		case 1:
		case 2:
			if (count > 0)
				fprintf(out, "|");
			fprintf(out, "%s", map_name);
			count++;
			break;
		default:
			continue;
		}
	}

	fprintf(out,
		// ")\\b</string>\n"
		// "				</dict>\n"
		")"MATCH_END"\"\n"
		"				}\n"
	);

	return;
}


bool vscodesyntax_script_add_builtin(const struct script_function *buildin, bool override)
{
	nullpo_retr(false, buildin);

	if (strncmp("__", buildin->name, 2) == 0) // Skip internal commands
		return false;

	if (buildin->deprecated != deprecated_cmd)
		return false;
	
	if (_cmd_count > 0)
		fprintf(out, "|");
	fprintf(out, "%s", buildin->name);
	_cmd_count++;

	return false;
}

void vscodesyntax_scriptcmd(void)
{
	bool(*script_add_builtin) (const struct script_function *buildin, bool override) = NULL;

	/* Link */
	script_add_builtin = script->add_builtin;
	script->add_builtin = vscodesyntax_script_add_builtin;

	deprecated_cmd = false;
	_cmd_count = 0;

	/* Run */
	fprintf(out,
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Script Commands</string>\n"
		// "					<key>name</key>\n"
		// "					<string>entity.name.function</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b("
		"				{\n"
		"					\"comment\": \"Script Commands\",\n"
		"					\"name\": \"entity.name.function\",\n"
		"					\"match\": \""MATCH_START"("
	);

	script->parse_builtin();

	fprintf(out,
		// ")\\b</string>\n"
		// "				</dict>\n"
		")"MATCH_END"\"\n"
		"				},\n"
	);

	deprecated_cmd = true;
	_cmd_count = 0;

	fprintf(out,
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Script Commands - Deprecated</string>\n"
		// "					<key>name</key>\n"
		// "					<string>invalid.deprecated</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b("
		"				{\n"
		"					\"comment\": \"Script Commands - Deprecated\",\n"
		"					\"name\": \"invalid.deprecated\",\n"
		"					\"match\": \""MATCH_START"("
	);

	script->parse_builtin();

	fprintf(out,
		// ")\\b</string>\n"
		// "				</dict>\n"
		")"MATCH_END"\"\n"
		"				}\n"
	);

	/* Unlink */
	script->add_builtin = script_add_builtin;
}

// ---------------------------
// 

void vscodesyntax_write_start(void)
{
	nullpo_retv(out);

	fprintf(out,
		// "<?xml version=\"1.0\" encoding=\"UTF - 8\"?>\n"
		// "<!DOCTYPE plist PUBLIC \" -//Apple//DTD PLIST 1.0//EN\" \"http://www.apple.com/DTDs/PropertyList-1.0.dtd\">\n"
		// "<plist version=\"1.0\">\n"
		// "<dict>\n"
		// "	<key>name</key>\n"
		// "	<string>Hercules Script</string>\n"
		// "	<key>firstLineMatch</key>\n"
		// "	<string>^//================= Hercules Script =======================================$</string>\n"
		// "	<key>scopeName</key>\n"
		// "	<string>source.hercscript</string>\n"
		"{\n"
		"	\"$schema\": \"https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json\",\n"
		"	\"name\": \"Hercules Script\",\n"
		"	\"scopeName\": \"source.herc\",\n"
		"	\"patterns\": [\n"
		"		{\n"
		"			\"include\": \"#comments\"\n"
		"		},\n"
		"		{\n"
		"			\"include\": \"#keywords\"\n"
		"		},\n"
		"		{\n"
		"			\"include\": \"#strings\"\n"
		"		},\n"
		"		{\n"
		"			\"include\": \"#herc_constants\"\n"
		"		},\n" // before numbers or 1_M_01 for example will be <number><constant>
		"		{\n"
		"			\"include\": \"#numbers\"\n"
		"		},\n"
		"		{\n"
		"			\"include\": \"#script_cmds\"\n"
		"		},\n"
		"		{\n"
		"			\"include\": \"#maps\"\n"
		"		},\n"
		"		{\n"
		"			\"include\": \"#vars\"\n"
		"		}\n"
		"	],\n"
		"	\"repository\": {\n"
	);
}

void vscodesyntax_write_main_patterns(void)
{
	nullpo_retv(out);

	fprintf(out,
		// "	<key>patterns</key>\n"
		// "	<array>\n"
		// "		<dict>\n"
		// "			<key>comment</key>\n"
		// "			<string>Single Line Comments</string>\n"
		// "			<key>name</key>\n"
		// "			<string>comment.line.double-slash</string>\n"
		// "			<key>match</key>\n"
		// "			<string>//.*$</string>\n"
		// "		</dict>\n"
		// "		<dict>\n"
		// "			<key>comment</key>\n"
		// "			<string>Multi Line Comments</string>\n"
		// "			<key>name</key>\n"
		// "			<string>comment.block</string>\n"
		// "			<key>begin</key>\n"
		// "			<string>/\\*.*</string>\n"
		// "			<key>end</key>\n"
		// "			<string>\\*/</string>\n"
		// "		</dict>\n"
		// "		<dict>\n"
		// "			<key>include</key>\n"
		// "			<string>#herc_constants</string>\n"
		// "		</dict>\n"
		// "		<dict>\n"
		// "			<key>include</key>\n"
		// "			<string>#lang_constants</string>\n"
		// "		</dict>\n"
		// "		<dict>\n"
		// "			<key>include</key>\n"
		// "			<string>#lang_keywords</string>\n"
		// "		</dict>\n"
		// "		<dict>\n"
		// "			<key>include</key>\n"
		// "			<string>#maps</string>\n"
		// "		</dict>\n"
		// "		<dict>\n"
		// "			<key>include</key>\n"
		// "			<string>#scriptcmds</string>\n"
		// "		</dict>\n"
		// "	</array>\n"
		// "	<key>repository</key>\n"
		// "	<dict>\n"
		"		\"comments\": {\n"
		"			\"patterns\": [\n"
		"				{\n"
		"					\"comment\": \"Single Line Comments\",\n"
		"					\"name\": \"comment.line.double-slash\",\n"
		"					\"match\": \"//.*$\"\n"
		"				},\n"
		"				{\n"
		"					\"comment\": \"Multi Line Comments\",\n"
		"					\"name\": \"comment.block\",\n"
		"					\"begin\": \"/\\\\*\",\n"
		"					\"end\": \"\\\\*/\"\n"
		"				}\n"
		"			]\n"
		"		},\n"
	);
}

void vscodesyntax_lang_constants(void)
{
	nullpo_retv(out);

	fprintf(out,
		// "		<key>lang_constants</key>\n"
		// "		<dict>\n"
		// "			<key>patterns</key>\n"
		// "			<array>\n"
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Strings</string>\n"
		// "					<key>name</key>\n"
		// "					<string>string.quoted</string>\n"
		// "					<key>begin</key>\n"
		// "					<string>\"</string>\n"
		// "					<key>end</key>\n"
		// "					<string>\"</string>\n"
		// "					<key>patterns</key>\n"
		// "					<array>\n"
		// "						<dict>\n"
		// "							<key>name</key>\n"
		// "							<string>constant.character.escape</string>\n"
		// "							<key>match</key>\n"
		// "							<string>\\\\.</string>\n"
		// "						</dict>\n"
		// "						<dict>\n"
		// "							<key>name</key>\n"
		// "							<string>constant.rgb-value</string>\n"
		// "							<key>match</key>\n"
		// "							<string>\\^[0-9a-fA-F]{6}</string>\n"
		// "						</dict>\n"
		// "					</array>\n"
		// "				</dict>\n"
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Numerical Constants</string>\n"
		// "					<key>name</key>\n"
		// "					<string>constant.numeric</string>\n"
		// "					<key>match</key>\n"
		// "					<string>[0-9][0-9]*</string>\n"
		// "				</dict>\n"
		// "			</array>\n"
		// "		</dict>\n"
"		\"strings\": {\n"
"			\"name\": \"string.quoted.double.hercsc\",\n"
"			\"begin\": \"\\\"\",\n"
"			\"end\": \"\\\"\",\n"
"			\"patterns\": [\n"
"				{\n"
"					\"name\": \"constant.rgb-value\",\n"
"					\"match\": \"\\\\^[0-9a-fA-F]{6}\"\n"
"				},\n"
"				{\n"
"					\"name\": \"constant.character.escape.hercsc\",\n"
"					\"match\": \"\\\\\\\\.\"\n"
"				}\n"
"			]\n"
"		},\n"
"		\"numbers\": {\n"
"			\"name\": \"constant.numeric\",\n"
"			\"patterns\": [\n"
"				{\n"
"					\"name\": \"constant.numeric\",\n"
"					\"match\": \"0x[0-9A-Fa-f]+\"\n"
"				},\n"
"				{\n"
"					\"name\": \"constant.numeric\",\n"
"					\"match\": \"\\\\d+\"\n"
"				}\n"
"			]\n"
"		},\n"
	);
}

void vscodesyntax_lang_keywords(void)
{
	nullpo_retv(out);

	fprintf(out,
		// "		<key>lang_keywords</key>\n"
		// "		<dict>\n"
		// "			<key>patterns</key>\n"
		// "			<array>\n"
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Keywords - Conditional</string>\n"
		// "					<key>name</key>\n"
		// "					<string>keyword.control.conditional</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b(if|else|switch)\\b</string>\n"
		// "				</dict>\n"
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Keywords - Repeat</string>\n"
		// "					<key>name</key>\n"
		// "					<string>keyword.control</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b(do|for|while)\\b</string>\n"
		// "				</dict>\n"
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>NPC Types Keywords</string>\n"
		// "					<key>name</key>\n"
		// "					<string>keyword.other</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b(boss_monster|monster|warp|script|trader|shop|mapflag|duplicate\\(.*\\))\\t</string>\n"
		// "				</dict>\n"
		// "				<dict>\n"
		// "					<key>comment</key>\n"
		// "					<string>Function declare</string>\n"
		// "					<key>name</key>\n"
		// "					<string>entity.name.function</string>\n"
		// "					<key>match</key>\n"
		// "					<string>(?i)\\b(function)\\t(script)\\t</string>\n"
		// "				</dict>\n"
		// "			</array>\n"
		// "		</dict>\n"
		"		\"keywords\": {\n"
		"			\"patterns\": [\n"
		"				{\n"
		"					\"name\": \"keyword.control.conditional\",\n"
		"					\"match\": \""MATCH_START"(if|else|switch|case)"MATCH_END"\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"keyword.control.repeat\",\n"
		"					\"match\": \""MATCH_START"(do|while|for)"MATCH_END"\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"keyword.control.other\",\n"
		"					\"match\": \""MATCH_START"(break|continue)"MATCH_END"\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"keyword.control.other\",\n"
		"					\"match\": \""MATCH_START"(boss_monster|monster|warp|script|trader|shop|mapflag|duplicate\\\\(.*\\\\))\\t\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"keyword.control.other\",\n"
		"					\"match\": \""MATCH_START"(function)\\t(script)\\t\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"entity.name.label\",\n"
		"					\"match\": \"\\\\w+:\"\n"
		"				}\n"
		"			]\n"
		"		},\n"

		// Operators
		"		\"operators\": {\n"
		"			\"patterns\": [\n"
		"				{\n"
		"					\"name\": \"keyword.operator\",\n"
		"					\"match\": \""MATCH_START"(>=|>|<=|<|==|!=|\\\\||&&|&)"MATCH_END"\"\n"
		"				}\n"
		"			]\n"
		"		},\n"

		// Variables
		"		\"vars\": {\n"
		"			\"patterns\": [\n"
		"				{\n"
		"					\"name\": \"variable.other.tempchar\",\n"
		"					\"match\": \"\\\\@\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.global\",\n"
		"					\"match\": \"\\\\$\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.tempglobal\",\n"
		"					\"match\": \"\\\\$\\\\@\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.npc\",\n"
		"					\"match\": \"\\\\.\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.scope\",\n"
		"					\"match\": \"\\\\.\\\\@\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.instance\",\n"
		"					\"match\": \"\\\\'\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.account\",\n"
		"					\"match\": \"##?\\\\w+\\\\$?\"\n"
		"				},\n"
		"				{\n"
		"					\"name\": \"variable.other.char\",\n"
		"					\"match\": \"\\\\w*[A-Za-z]\\\\w*\\\\$?\"\n"
		"				}\n"
		"			]\n"
		"		},\n"
	);
}

bool vscodesyntax_putfile(const char *filename, const char *label)
{
	FILE *src;

	nullpo_retr(false, filename);
	nullpo_retr(false, label);
	nullpo_retr(false, out);

	fprintf(out,
		// "\t\t<key>%s</key>\n"
		// "\t\t<dict>\n"
		// "\t\t\t<key>patterns</key>\n"
		// "\t\t\t<array>\n",
		"		\"%s\": {\n"
		"			\"patterns\": [\n",
		label
	);

	if ((src = fopen(filename, "rt")) == NULL) {
		fprintf(stderr, "Failed to open temporary file\n");
		return false;
	}

	while (fgets(_buffer, BUFFER_SIZE, src) != NULL) {
		fprintf(out, "%s", _buffer);
	}

	fprintf(out,
		// "\t\t\t</array>\n"
		// "\t\t</dict>\n"
		"			]\n"
		"		},\n"
	);

	fclose(src);

	return true;
}

// By Haru
bool createdirectory(const char *dirname)
{
#ifdef WIN32
	if (!CreateDirectory(dirname, NULL)) {
		if (ERROR_ALREADY_EXISTS != GetLastError())
			return false;
	}
#else /* Not WIN32 */
	struct stat st = { 0 };
	if (stat(dirname, &st) == -1) {
		if (mkdir(dirname, 0755) != 0)
			return false;
	}
#endif // WIN32 check
	return true;
}

/**
 * Closes current out file and create/open a new one with filename
 */
bool vscodesyntax_redirect_out(const char * filename)
{
	char fn[255];

	nullpo_ret(filename);
	
	if (out != NULL)
		fclose(out);

	sprintf(fn, "vscodeplugin/%s", filename);

	if ((out = fopen(fn, "w+t")) == NULL) {
		fprintf(stderr, "Failed to create output file\n");
		return false;
	}

	return true;
}

/**
 * Main generation routine
 * - calls other generation routines
 */
void vscodesyntax_generate(void)
{
	if (createdirectory("vscodeplugin") == false || createdirectory("vscodeplugin/temp") == false) {
		fprintf(stderr, "Failed to create output directory\n");
		return;
	}

	// Write temporary files
	// Const DB
	if (!vscodesyntax_redirect_out("temp/const.js"))
		return;
	vscodesyntax_constdb();

	// Maps DB
	if (!vscodesyntax_redirect_out("temp/maps.js"))
		return;
	vscodesyntax_mapdb();

	// Script Commands
	if (!vscodesyntax_redirect_out("temp/scriptcmd.js"))
		return;
	vscodesyntax_scriptcmd();


	// Generate final output with temporary ones
	if (!vscodesyntax_redirect_out("hercscript.tmLanguage.js"))
		return;

	vscodesyntax_write_start();

	vscodesyntax_write_main_patterns();

	if (!vscodesyntax_putfile("vscodeplugin/temp/const.js", "herc_constants"))
		return;
	
	vscodesyntax_lang_constants();
	vscodesyntax_lang_keywords();
	
	if (!vscodesyntax_putfile("vscodeplugin/temp/maps.js", "maps"))
		return;
	if (!vscodesyntax_putfile("vscodeplugin/temp/scriptcmd.js", "script_cmds"))
		return;
	fseek(out, -2, SEEK_CUR);

	fprintf(out, "\n\t}\n}\n");
	// fprintf(out, "\n\t</dict>\n</dict>\n");

	fclose(out);
}

// Console Commands: vscode_syntax 
CPCMD(vscode_syntax)
{
	vscodesyntax_generate();
}

// Parameter: --vscode_syntax
CMDLINEARG(vscode_syntax)
{
	map->minimal = torun = true;
	return true;
}

// Server Pre-Init
// register parameters
HPExport void server_preinit(void)
{
	addArg("--vscode_syntax", false, vscode_syntax, NULL);
}

// Plugin Init
// Register console commands
HPExport void plugin_init(void)
{
	addCPCommand("server:tools:vscode_syntax", vscode_syntax);
}

// Server Online
// Saves the generated vscode syntax
HPExport void server_online(void)
{
	if (torun)
		vscodesyntax_generate();
}