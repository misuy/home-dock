<template>
    <div class="storage_area" :key="$route.path + entry_type">
        <div class="header">
            <img class="logo" src="../../public/logo.png" height="100"/>
            <div class="name">home-dock</div>
            <div class="path">{{ storage_path }}</div>
        </div>
        <div class="entries_area">
            <EntryNotFound v-if="is_entry_null"/>
            <DirArea v-if="is_entry_dir" :input_dir="entry.cast_to_dir()" @change-path="change_path"/>
            <ImgArea v-if="is_entry_img" :input_img="entry.cast_to_img()"/>
            <FileArea v-if="is_entry_file" :input_file="entry.cast_to_file()"/>
        </div>
    </div>
</template>

<script lang="ts">
import DirArea from "./DirArea.vue";
import ImgArea from "./ImgArea.vue";
import FileArea from "./FileArea.vue";
import EntryNotFound from "./EntryNotFound.vue";
import { EntryType, StorageEntry } from "@/util";
import { api_call_check_entry_type } from "@/api_util"
import { defineComponent } from "vue";

export default defineComponent({
    name: 'StorageArea',
    props: {
        base_url: {
            type: String,
            required: true,
        },
    },
    components: {
        DirArea,
        ImgArea,
        FileArea,
        EntryNotFound,
    },
    data() {
        return {
            entry_type: EntryType.NULL,
        }
    },
    methods: {
        change_path(new_path: String) {
            this.$router.push({ path: this.base_url + "/" + new_path.toString() });
        },
        set_entry_type(type: EntryType) {
            this.entry_type = type;
        },
        check_entry_type() {
            api_call_check_entry_type(this.storage_path, this.set_entry_type);
        },
    },
    computed: {
        is_entry_null() {
            return this.entry_type == EntryType.NULL;
        },
        is_entry_dir() {
            return this.entry_type == EntryType.Dir;
        },
        is_entry_file() {
            return this.entry_type == EntryType.File;
        },
        is_entry_img() {
            return this.entry_type == EntryType.Jpg || this.entry_type == EntryType.Png;
        },
        entry() {
            return new StorageEntry(this.storage_path, this.entry_type);
        },
        storage_path() {
            let path: string = this.$route.path.split(this.base_url)[1];
            if (path[0] != "/") path = "/" + path;
            console.log("--", path)
            return path;
        },
    },
    watch: {
        storage_path() {
            this.check_entry_type();
        }
    },
    created() {
        this.check_entry_type();
    },
})
</script>

<style>
    .storage_area {
        width: 100%;
        height: 100%;

        display: grid;
        grid-template-rows: minmax(10%, 100px) auto;
    }

    .header {
        grid-row: 1;
        width: 100%;

        display: grid;
        grid-template-columns: min-content max-content 1fr;
        align-items: center;
        text-align: center;
    }

    .logo {
        grid-column: 1;
    }

    .name {
        grid-column: 2;
        font-weight: bold;
        font-size: larger;
    }

    .path {
        grid-column: 3;
        font-weight: bold;
        font-size: large;
    }

    .entries_area {
        grid-row: 2;
    }

    button {
        background-color: #8fbeff;
        padding: 5px 5px;
        text-decoration: none;
        border: 2px #8fbeff;
        border-radius: 5px;
    }

    button:hover {
        background-color: #4d89dc;
    }
</style>