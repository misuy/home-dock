const API_URL: string = "http://localhost:8000/home_dock_api"
const READ_DIR_URL: string = "/read_dir";
const CREATE_DIR_URL: string = "/create_dir";
const READ_FILE_URL: string = "/read_file";
const WRITE_FILE_URL: string = "/write_file";
const CHECK_ENTRY_TYPE_URL: string = "/check_entry_type";


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


export function api_call_check_entry_type(path: string, success_callback: (type: EntryType) => void) {
    $.ajax({
        url: API_URL + CHECK_ENTRY_TYPE_URL + path,
        method: "get",
        dataType: "json",
        success: function(data) {
            let entry_type;
            switch (data) {
                case ("dir"): { entry_type = EntryType.Dir; break; }
                case ("file"): {
                    const splitted_by_dot = path.split(".");
                    if (splitted_by_dot.length > 1) {
                        switch (splitted_by_dot[splitted_by_dot.length-1]) {
                            case ("jpg"): { entry_type = EntryType.Jpg; break; }
                            case ("png"): { entry_type = EntryType.Png; break; }
                            default: entry_type = EntryType.File;
                        }
                    }
                    else entry_type = EntryType.File;
                    break;
                }
                default: entry_type = EntryType.NULL;
            }
            success_callback(entry_type);
        },
    })
}