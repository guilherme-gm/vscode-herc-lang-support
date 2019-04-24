import { TextDocument } from "vscode-languageserver";
export type DocumentEvent = (document: TextDocument) => any;
export type ConfigEvent = (settings: any) => any;
