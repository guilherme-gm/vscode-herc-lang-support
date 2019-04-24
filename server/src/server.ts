import {
	createConnection,
	ProposedFeatures,
	InitializeParams,
	DidChangeConfigurationNotification,
} from 'vscode-languageserver';
import * as completionProvider from "./providers/completionProvider";
import * as signatureProvider from "./providers/signatureProvider";
import * as diagProvider from "./providers/diagnosticProvider";
import * as documentsManager from "./helpers/documentsManager";
import * as conf from "./helpers/configManager";

// Create a connection for the server. The connection uses Node's IPC as a transport.
// Also include all preview / proposed LSP features.
export const connection = createConnection(ProposedFeatures.all);

// Init Managers
documentsManager.init(connection);
conf.init(connection);

// Init Providers
diagProvider.init();

/**
 * https://microsoft.github.io/language-server-protocol/specification#initialize
 */
connection.onInitialize((params: InitializeParams) => {
	conf.updateCapabilities(params.capabilities);

	// Tells the client all features supported by server
	return {
		capabilities: {
			textDocumentSync: documentsManager.syncKind,
			completionProvider: {
				resolveProvider: true
			},
			signatureHelpProvider: {
				triggerCharacters: [
					"(", ","
				]
			},
		}
	};
});

connection.onCompletion(completionProvider.getCompletions); // Returns code completion for current context
connection.onCompletionResolve(completionProvider.getCompletionInfo); // Information about selected completion
connection.onSignatureHelp(signatureProvider.getSignature); // Returns signature for current context

connection.onInitialized(() => {
	if (conf.hasConfigurationCapability) {
		// Register for all configuration changes.
		connection.client.register(DidChangeConfigurationNotification.type, undefined);
	}
	if (conf.hasWorkspaceFolderCapability) {
		connection.workspace.onDidChangeWorkspaceFolders(_event => {
			connection.console.log('Workspace folder change event received.');
		});
	}
});

// Listen on the connection
connection.listen();
