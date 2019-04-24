import { TextDocuments, Connection, TextDocumentIdentifier, TextDocument } from "vscode-languageserver";
import { DocumentEvent } from "./events";

// Create a simple text document manager. The text document manager
// supports full document sync only
export let documents: TextDocuments = new TextDocuments();

export let didChangeSubs: DocumentEvent[] = [];
export let didSaveSubs: DocumentEvent[] = [];

export const syncKind = documents.syncKind;

export function init(connection: Connection) {
	/* Register Events Here */

	// The content of a text document has changed. This event is emitted when the text document first opened or when its content has changed.
	documents.onDidChangeContent(change => { didChangeSubs.forEach(sub => { sub(change.document); }); });

	// The content of a text document was saved.
	documents.onDidSave(save => { didSaveSubs.forEach(sub => sub(save.document)) });

	// Make the text document manager listen on the connection
	// for open, change and close text document events
	documents.listen(connection);
}

export function all(): TextDocument[] {
	return documents.all();
}

export function getText(uri: TextDocumentIdentifier): string {
	return documents.get(uri.uri).getText();
}
