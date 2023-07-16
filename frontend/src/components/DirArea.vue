<template>
    <div class="dir_area">
        <input type="file" class="new_files_selector" ref="new_files_selector" multiple="true" @change="update_new_files_list"/>
        <button class="new_files_btn" @click="fire_new_files_selector">добавить файлы</button>
        <button class="upload_new_files" @click="upload_new_files">upload</button>
        <EntryBox v-for="/* eslint-disable-line */ entry in dir.entries" :entry="entry" @change-path="emit_change_path"/>
        <p v-for="/* eslint-disable-line */ new_file in new_files_list">{{ new_file.name }}</p>
    </div>
</template>

<script lang="ts">
import EntryBox from './EntryBox.vue';
import { defineComponent } from 'vue';
import { Dir } from "../util"
import { api_call_create_dir } from "../api_util"
import { File } from '../util';
import { StorageEntry } from '../util';
import { EntryType } from '../util';

export default defineComponent({
    name: "DirArea",
    components: {
        EntryBox,
    },
    props: {
        input_dir: {
            type: Dir,
            required: true,
        }
    },
    data() {
        return {
            dir: this.input_dir,
            new_files_list: new Array<File>(),
        }
    },
    emits: ["change-path"],
    methods: {
        emit_change_path(new_path: String) {
            this.$emit("change-path", new_path);
        },
        create_dir() {
            api_call_create_dir(this.dir.path + "/new", () => this.dir.load_entries());
        },
        fire_new_files_selector() {
            (this.$refs.new_files_selector as HTMLInputElement).click();
        },
        update_new_files_list(event: Event) {
            let event_files = (event.target as HTMLInputElement).files as FileList;
            for (let i=0; i<event_files.length; i++) {
                let event_file = event_files[i];
                let new_file = new File(new StorageEntry(this.dir.path + "/" + event_file.name, EntryType.File));
                event_file.arrayBuffer().then((buffer) => {
                    new_file.set_data(new Uint8Array(buffer));
                    this.new_files_list.push(new_file);
                })
            }
        },
        upload_new_files() {
            for (let new_file of this.new_files_list) {
                new_file.write_data((type) => {
                    if (type == EntryType.NULL) alert(new_file.name + "не загружен");
                    this.new_files_list.splice(this.new_files_list.indexOf(new_file), 1);
                    this.dir.load_entries();
                });
            }
        },
    },
    created() {
        this.dir.load_entries();
    },
})
</script>

<style scoped>
    .dir_area {
        width: 100%;
    }

    .new_files_selector {
        display: none;
    }
</style>