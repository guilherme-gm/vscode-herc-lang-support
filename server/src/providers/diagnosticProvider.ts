import * as conf from "../helpers/configManager";
import * as docMngr from "../helpers/documentsManager";
import { connection } from "../server";
import { TextDocument, Diagnostic, DiagnosticSeverity } from "vscode-languageserver";
import Uri from 'vscode-uri'

//let mapserverFolderPath = '';
let fileInProgress: string = '';

export function init() {
    conf.configChangedSubs.push(configChanged);
    docMngr.didSaveSubs.push(updateDiagnostics);

    //const collection = vscode.languages.createDiagnosticCollection('hercscript');
}

function configChanged(settings) {
    docMngr.all().forEach((doc: TextDocument) => {
        updateDiagnostics(doc);
    });

    /*
    Maybe we can run this only on map server changes? But what if there are 2 files in different workspaces?
    No idea how this works...
    if (settings.hercscript.mapServerPath.toString().trim() == mapserverFolderPath) {
        return;
    }

    mapserverFolderPath = settings.hercscript.mapserverPath.toString().trim();

    if (mapserverFolderPath != null) {
    
    }
    */
}

async function updateDiagnostics(document: TextDocument): Promise<void> {
    let hasErrors = false;
    let settings = await conf.get(document.uri);

    const child_process = require('child_process');

    fileInProgress = document.uri; // !FIXME : This only works for 1 file
    var mapserver = child_process.spawn(
        settings.mapserverFolderPath + "map-server", ["--load-script", Uri.parse(document.uri).fsPath, '--script-check'],
        { "cwd": settings.mapserverFolderPath }
    );

    mapserver.on('error', function (error) {
        connection.console.log("Error: bad command:" + error);
        // TODO : Change to a notification
    });

    mapserver.stderr.on('data', function (data) {
        let errors = data.toString().split('[Error]');
        errors.shift();
        hasErrors = true;

        let diagnostics: Diagnostic[] = [];
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

            // !FIXME : This is mostly wrong
            diagnostics.push({
                code: '',
                message: err,
                range: { start: { line: errorLine, character: 0 }, end: { line: errorLine, character: 3 } },
                severity: DiagnosticSeverity.Error,
                source: '',
                relatedInformation: [

                ]
            });
        });

        connection.sendDiagnostics({ uri: fileInProgress, diagnostics: diagnostics });
    });

    mapserver.on('exit', function () {
        if (hasErrors === false) {
            connection.sendDiagnostics({ uri: fileInProgress, diagnostics: [] });
        }
    });
}