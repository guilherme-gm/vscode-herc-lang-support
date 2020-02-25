/* --------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See License.txt in the project root for license information.
 * ------------------------------------------------------------------------------------------ */

import * as path from 'path';
import { workspace, ExtensionContext, window } from 'vscode';

import {
	LanguageClient,
	LanguageClientOptions,
	ServerOptions,
	TransportKind,
	Executable
} from 'vscode-languageclient';

let client: LanguageClient;

export function activate(context: ExtensionContext) {
	// If the extension is launched in debug mode then the debug server options are used
	// Otherwise the run options are used
	let run: Executable = {
		command: path.resolve("server", "target", "debug", "server"),
		options: { cwd: "" }
	};
	let serverOptions: ServerOptions = {
		run,
		debug: run
	};

	// Options to control the language client
	let clientOptions: LanguageClientOptions = {
		// Register the server for plain text documents
		documentSelector: [{ scheme: 'file', language: 'hercscript' }],
		initializationOptions: { script_cmd: path.resolve("client", "out", "commands.json") }, // FIXME: Is this path correct for production?
	};
	
/*  ---- Find out Tab-Size -----
	console.log(JSON.stringify(workspace.getConfiguration("editor").get("tabSize")));
	console.log(window.activeTextEditor.options.tabSize);
	window.onDidChangeTextEditorOptions((e) => {
		console.log(e);
		console.log(JSON.stringify(workspace.getConfiguration("editor").get("tabSize")));
	})
	----- ----------------------
*/	
	console.log(path.resolve());
	// Create the language client and start the client.
	client = new LanguageClient(
		'languageServerExample',
		'Language Server Example',
		serverOptions,
		clientOptions
	);

	// Start the client. This will also launch the server
	client.start();
	console.log("Starting");
}

export function deactivate(): Thenable<void> | undefined {
	if (!client) {
		return undefined;
	}
	return client.stop();
}
