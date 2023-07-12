export enum EntryType {
    File,
    Dir,
    NULL,
}

export class StorageEntry {
    path: String;
    type: EntryType;

    constructor (path: String, type: EntryType) {
        this.path = path;
        this.type = type;
    }

    is_dir(): boolean {
        return this.type == EntryType.Dir;
    }
}


export class File {
    path: String;

    constructor (entry: StorageEntry) {
        this.path = entry.path;
    }

    get name(): String|undefined {
        return this.path.split("/").pop() + " file";
    }
}


export class Dir {
    path: String;

    constructor (entry: StorageEntry) {
        this.path = entry.path;
    }

    get name(): string|undefined {
        return this.path.split("/").pop() + " dir";
    }
}