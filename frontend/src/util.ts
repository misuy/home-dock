import { api_call_check_entry_type, api_call_read_dir, api_call_read_file } from "./api_util";

export enum EntryType {
    File,
    Dir,
    Jpg,
    Png,
    NULL,
}

export class StorageEntry {
    path: String;
    type: EntryType;

    constructor (path: String, type: EntryType) {
        this.path = path;
        this.type = type;
    }

    if_null(): boolean {
        return this.type == EntryType.NULL;
    }

    is_dir(): boolean {
        return this.type == EntryType.Dir;
    }

    if_img(): boolean {
        return this.type == EntryType.Png || this.type == EntryType.Jpg;
    }

    is_file(): boolean {
        return this.type == EntryType.File;
    }

    get name(): string {
        return this.path.split("/").pop() + "";
    }

    load_type() {
        api_call_check_entry_type(this.path.toString(), (type) => { this.type = type; });
    }

    cast_to_dir(): Dir {
        return new Dir(this);
    }

    cast_to_file(): File {
        return new File(this);
    }

    cast_to_img(): Img {
        return new Img(this);
    }
}

export class Dir extends StorageEntry {
    entries: Array<StorageEntry>;

    constructor (entry: StorageEntry) {
        super(entry.path, entry.type);
        this.entries = new Array();
    }

    load_entries() {
        api_call_read_dir(this.path.toString(), (entries) => this.entries = entries);
    }
}

export class File extends StorageEntry {
    data: Uint8Array|undefined;
    url: string|undefined;

    constructor (entry: StorageEntry) {
        super(entry.path, entry.type);
        this.data = undefined;
        this.url = undefined;
    }

    load_data() {
        if (this.data == undefined) api_call_read_file(this.path.toString(), (content) => { this.data = content; } );
    }

    use_data(callback: (data: Uint8Array) => void) {
        if (this.data == undefined) api_call_read_file(this.path.toString(), (content) => { this.data = content; callback(this.data); } );
        else callback(this.data);
    }

    create_url() {
        if (this.url == undefined) {
            this.get_blob((blob) => { this.url = URL.createObjectURL(blob); })
        }
    }

    use_url(callback: (url: string) => void) {
        if (this.url == undefined) {
            this.get_blob((blob) => { this.url = URL.createObjectURL(blob); callback(this.url); })
        }
        else callback(this.url);
    }

    get_blob(callback: (data: Blob) => void) {

        this.use_data((data) => {
            let type;
            switch (this.type) {
                case (EntryType.Jpg): { type = "image/jpeg"; break; }
                case (EntryType.Png): { type = "image/png"; break; }
                case (EntryType.File): { type = "text/plain"; break; }
                default: return;
            }
            callback(new Blob([data as Uint8Array], { type: type }))
        })
    }
    
}

export class Img extends File {
    url: string|undefined;
    
    constructor (entry: StorageEntry) {
        super(entry);
        this.url = undefined;
    }
}

export function save_file_by_url(url: string, file_name: string) {
    const dwnld_link = document.createElement("a");
    dwnld_link.href = url;
    dwnld_link.download = file_name;
    document.body.appendChild(dwnld_link);
    dwnld_link.click();
    document.body.removeChild(dwnld_link);
}