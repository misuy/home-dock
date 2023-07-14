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

    get name(): string|undefined {
        return this.path.split("/").pop() + (this.is_dir() ? " dir" : " file");
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

    constructor (entry: StorageEntry) {
        super(entry.path, entry.type);
        this.data = undefined;
    }

    load_data(callback: () => void) {
        if (this.data == undefined) api_call_read_file(this.path.toString(), (content) => { this.data = content; callback(); } );
        else callback();
    }
}

export class Img extends File {
    url: string|undefined;
    
    constructor (entry: StorageEntry) {
        super(entry);
        this.url = undefined;
    }

    create_url() {
        if (this.url == undefined) {
            this.get_img_blob((blob) => { this.url = URL.createObjectURL(blob); })
        }
    }

    get_img_blob(callback: (data: Blob) => void) {

        super.load_data(() => {
            let img_type;
            switch (this.type) {
                case (EntryType.Jpg): { img_type = "jpg"; break; }
                case (EntryType.Png): { img_type = "png"; break; }
                default: return;
            }
            callback(new Blob([this.data as Uint8Array], { type: "image/" + img_type }))
        })
    }
}