'use strict';

import * as vscode from 'vscode';
import { registerCompletionProviders } from "./completionProvider";

export function activate(context: vscode.ExtensionContext) {
	registerCompletionProviders(context);
}