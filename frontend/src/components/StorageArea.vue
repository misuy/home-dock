<template>
    <div class="storage_area" :key="$route.path + entry_type">
        <DirArea v-if="is_entry_dir" :input_dir="entry.cast_to_dir()" @change-path="change_path"/>
        <ImgArea v-if="is_entry_img" :input_img="entry.cast_to_img()"/>
        <FileArea v-if="is_entry_file" :input_file="entry.cast_to_file()"/>
    </div>
</template>

<script lang="ts">
import DirArea from "./DirArea.vue";
import ImgArea from "./ImgArea.vue";
import FileArea from "./FileArea.vue";
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
        width: 500px;
    }
</style>