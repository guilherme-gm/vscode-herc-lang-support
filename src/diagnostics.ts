import * as vscode from 'vscode';
import { conf } from "./extension";

export function register(context: vscode.ExtensionContext) {
    const mapserverFolderPath = conf.get('mapserverFolderPath').toString().trim();

    if (mapserverFolderPath === '') {
        return; // code diagnostics disabled
    }

    const collection = vscode.languages.createDiagnosticCollection('hercscript');
    
    if (vscode.window.activeTextEditor) {
        updateDiagnostics(vscode.window.activeTextEditor.document, collection);
    }

    context.subscriptions.push(vscode.window.onDidChangeActiveTextEditor(e => updateDiagnostics(e.document, collection)));
}

function updateDiagnostics(document: vscode.TextDocument, collection: vscode.DiagnosticCollection): void {
    const mapserverFolderPath = conf.get('mapserverFolderPath').toString().trim();

    let hasErrors = false;
    if (document.uri.scheme !== 'file') {
        /* @TODO : Show a notification */
        return;
    }

    let child_process = require('child_process');

    var mapserver = child_process.spawn(
        mapserverFolderPath + "map-server", ["--load-script", document.uri.path, '--script-check'],
        { "cwd": mapserverFolderPath }
    );

    mapserver.on('error', function (error) {
        console.log("Error: bad command", error);
    });

    mapserver.stderr.on('data', function (data) {
        let errors = data.toString().split('[Error]');
        errors.shift();
        hasErrors = true;

        let errorList = [];
        errors.forEach(err => {
            const words = err.split(' ');
            let errorLine = -1;
            for (let i = 0; i < words.length; i++) {
                let match = null;
                if (words[i] === 'line' && words.length > i + 1 && ((match = words[i + 1].match(/'(\d+)'/)) !== null || (match = words[i + 1].match(/(\d+)/)) !== null)) {
                    errorLine = parseInt(match[1]) - 1;
                    break;
                }
            }

            if (errorLine == -1) {
                errorLine = 0;
            }

            errorList.push({
                code: '',
                message: err,
                //range: new vscode.Range(new vscode.Position(errorLine, 0), new vscode.Position(errorLine, 10)),
                range: document.lineAt(errorLine).range,
                severity: vscode.DiagnosticSeverity.Error,
                source: '',
                relatedInformation: [
                ]
            });
        });

        collection.set(document.uri, errorList);
    });

    mapserver.on('exit', function () {
        if (hasErrors === false) {
            collection.clear();
        }
    });
}