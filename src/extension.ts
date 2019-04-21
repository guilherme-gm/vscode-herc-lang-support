'use strict';

import * as vscode from 'vscode';
import { registerCompletionProviders } from "./completionProvider";
import * as diag from "./diagnostics";

export const conf = vscode.workspace.getConfiguration('hercscript');

export function activate(context: vscode.ExtensionContext) {
	registerCompletionProviders(context);
	diag.register(context);
}
