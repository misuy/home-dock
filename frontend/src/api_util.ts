const API_URL: string = "http://localhost:8000/home_dock_api"
const READ_DIR_URL: string = "/read_dir";
const CREATE_DIR_URL: string = "/create_dir";
const READ_FILE_URL: string = "/read_file";
const WRITE_FILE_URL: string = "/write_file";


import { StorageEntry, EntryType } from "./util";
import $ from "jquery";

class JSONStorageEntry {
    path: String;
    entry_type: String;

    constructor(path: String, entry_type: String) {
        this.path = path;
        this.entry_type = entry_type;
    }
}

function json_to_storage_entry(json: JSONStorageEntry): StorageEntry {
    let entry_type: EntryType;
    switch (json.entry_type) {
        case ("file"): { entry_type = EntryType.File; break; }
        case ("dir"): { entry_type = EntryType.Dir; break; }
        default: entry_type = EntryType.NULL;
    }
    return new StorageEntry(json.path, entry_type);
}

export function api_call_read_dir(path: string, success_callback: (entries: Array<StorageEntry>) => void) {
    $.ajax({
        url: API_URL + READ_DIR_URL + path,
        method: "get",
        dataType: "json",
        success: function(data) {
            const entries: Array<StorageEntry> = (data as Array<JSONStorageEntry>).map((value: JSONStorageEntry, index: number, array: JSONStorageEntry[]) => json_to_storage_entry(value));
            success_callback(entries);
        },
    })
}

export function api_call_read_file(path: string, success_callback: (content: Uint8Array) => void) {
    $.ajax({
        url: API_URL + READ_FILE_URL + path,
        method: "get",
        dataType: "json",
        success: function(data) {
            success_callback(new Uint8Array(data));
        }
    })
}

