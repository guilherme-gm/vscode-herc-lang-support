import { Connection, DidChangeConfigurationParams, ClientCapabilities } from "vscode-languageserver";
import { ConfigEvent } from "./events";

interface UserSettings {
    mapserverFolderPath: string
}

export let configChangedSubs: ConfigEvent[] = [];
let con: Connection = null;

export let hasConfigurationCapability: boolean = false;
export let hasWorkspaceFolderCapability: boolean = false;
export let hasDiagnosticRelatedInformationCapability: boolean = false;

// The global settings, used when the `workspace/configuration` request is not supported by the client.
// Doesn't happen in VS Code client, but is there for other clients that may be added later on.
const defaultSettings: UserSettings = {
    mapserverFolderPath: null
};
let globalSettings: UserSettings = defaultSettings;

// Cache the settings of all open documents
let documentSettings: Map<string, Thenable<UserSettings>> = new Map();

export function init(connection: Connection) {
    con = connection;
    connection.onDidChangeConfiguration(onDidChangeConfiguration);
}

export function updateCapabilities(capabilities: ClientCapabilities) {
    // Does the client support the `workspace/configuration` request?
    // If not, we will fall back using global settings
    hasConfigurationCapability = !!(
        capabilities.workspace && !!capabilities.workspace.configuration
    );
    hasWorkspaceFolderCapability = !!(
        capabilities.workspace && !!capabilities.workspace.workspaceFolders
    );
    hasDiagnosticRelatedInformationCapability = !!(
        capabilities.textDocument &&
        capabilities.textDocument.publishDiagnostics &&
        capabilities.textDocument.publishDiagnostics.relatedInformation
    );
}

function onDidChangeConfiguration(change: DidChangeConfigurationParams) {
    if (hasConfigurationCapability) {
        documentSettings.clear(); // Reset all cached document settings
    } else {
        globalSettings = <UserSettings>(
            (change.settings.hercscript || defaultSettings)
        );
    }

    configChangedSubs.forEach(sub => { sub(change.settings) })
}

export function get(settingName: string): Thenable<UserSettings> {
    if (!hasConfigurationCapability) {
        return Promise.resolve(globalSettings);
    }

    let result = documentSettings.get(settingName);
    if (!result) {
        result = con.workspace.getConfiguration({
            scopeUri: settingName,
            section: 'hercscript'
        });
        documentSettings.set(settingName, result);
    }

    return result;
}